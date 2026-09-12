//! Repository-scoped discovery, templates, durable links, and read-only Git history.

use crate::document::{Document, Record};
use pulldown_cmark::{Event, Parser, Tag};
use std::{
    ffi::OsStr,
    fmt, fs,
    path::{Component, Path, PathBuf},
    process::Command,
};

/// A machine-readable failure, shared by interactive and headless callers.
#[derive(Debug)]
pub struct Error {
    pub code: &'static str,
    pub message: String,
}
impl Error {
    pub(crate) fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
    /// Process status for headless callers.
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        match self.code {
            "usage" => 2,
            "conflict" => 3,
            "not_found" => 4,
            _ => 1,
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::new(
            if error.kind() == std::io::ErrorKind::NotFound {
                "not_found"
            } else {
                "io"
            },
            error.to_string(),
        )
    }
}
impl From<crate::document::Error> for Error {
    fn from(error: crate::document::Error) -> Self {
        Self::new(
            if matches!(error, crate::document::Error::Conflict(_)) {
                "conflict"
            } else {
                "invalid_document"
            },
            error.to_string(),
        )
    }
}

/// A resolved durable decision identity.
#[derive(Clone, Debug)]
pub struct Relationship {
    pub file: String,
    pub id: u64,
    pub title: String,
}
/// A Git revision and its subject line.
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub revision: String,
    pub summary: String,
}
/// One bounded page of history, with an explicit continuation offset.
#[derive(Clone, Debug)]
pub struct HistoryPage {
    pub entries: Vec<HistoryEntry>,
    pub has_more: bool,
    pub next_offset: usize,
}
/// Operations confined to a canonical repository root.
#[derive(Debug)]
pub struct Repository {
    root: PathBuf,
}
impl Repository {
    /// Open an existing directory.
    /// # Errors
    /// Fails when the directory does not exist or cannot be resolved.
    pub fn new(root: impl AsRef<Path>) -> Result<Self, Error> {
        let root = fs::canonicalize(root)?;
        if !root.is_dir() {
            return Err(Error::new("usage", "Root must be a directory"));
        }
        Ok(Self { root })
    }
    /// Resolve a relative Markdown path, rejecting traversal and external links.
    /// # Errors
    /// Rejects paths outside the root, non-Markdown paths, or absent parents.
    pub fn path(&self, file: &str) -> Result<PathBuf, Error> {
        let relative = Path::new(file);
        if relative.is_absolute()
            || relative
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
            || relative.as_os_str().is_empty()
        {
            return Err(Error::new(
                "invalid_path",
                "Use a root-relative path without dot or parent components",
            ));
        }
        if !markdown(relative) {
            return Err(Error::new(
                "invalid_path",
                "Decision files must end in .md or .markdown",
            ));
        }
        let candidate = self.root.join(relative);
        let resolved = match fs::symlink_metadata(&candidate) {
            Ok(_) => fs::canonicalize(&candidate)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let parent = candidate
                    .parent()
                    .ok_or_else(|| Error::new("invalid_path", "Missing parent"))?;
                fs::canonicalize(parent)?.join(
                    candidate
                        .file_name()
                        .ok_or_else(|| Error::new("invalid_path", "Missing filename"))?,
                )
            }
            Err(error) => return Err(error.into()),
        };
        if !resolved.starts_with(&self.root) {
            return Err(Error::new("invalid_path", "Path escapes repository root"));
        }
        Ok(resolved)
    }
    /// Return a root-relative UTF-8 path.
    /// # Errors
    /// Rejects paths outside the root or non-UTF-8 names.
    pub fn relative(&self, path: &Path) -> Result<String, Error> {
        path.strip_prefix(&self.root)
            .map_err(|_| Error::new("invalid_path", "Path escapes repository root"))?
            .to_str()
            .map(str::to_owned)
            .ok_or_else(|| Error::new("invalid_path", "Non-UTF-8 filename"))
    }
    /// Discover sorted Markdown documents, excluding hidden and generated directories.
    /// # Errors
    /// Reports unreadable or malformed documents instead of hiding errors.
    pub fn documents(&self) -> Result<Vec<Document>, Error> {
        let mut paths = Vec::new();
        discover(&self.root, &mut paths)?;
        paths.sort();
        paths
            .into_iter()
            .map(|path| Document::load(path).map_err(Error::from))
            .collect()
    }
    /// List names of repository templates in deterministic order.
    /// # Errors
    /// Reports unreadable template directories.
    pub fn templates(&self) -> Result<Vec<String>, Error> {
        let directory = self.root.join(".vrdx/templates");
        if !directory.exists() {
            return Ok(Vec::new());
        }
        if !fs::canonicalize(&directory)?.starts_with(&self.root) {
            return Err(Error::new(
                "invalid_path",
                "Template directory escapes repository",
            ));
        }
        let mut names = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if entry.file_type()?.is_file() && entry.path().extension() == Some(OsStr::new("md")) {
                names.push(
                    entry
                        .path()
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .ok_or_else(|| Error::new("invalid_path", "Non-UTF-8 template name"))?
                        .to_owned(),
                );
            }
        }
        names.sort();
        Ok(names)
    }
    /// Read one canonical record from a named repository template.
    /// # Errors
    /// Rejects invalid names, escaping paths, and templates with other record counts.
    pub fn template(&self, name: &str) -> Result<Record, Error> {
        if name.is_empty()
            || Path::new(name).components().count() != 1
            || name.contains(['/', '\\'])
            || name == "."
            || name == ".."
        {
            return Err(Error::new(
                "invalid_path",
                "Template name must be one filename stem",
            ));
        }
        let path = self.path(&format!(".vrdx/templates/{name}.md"))?;
        let document = Document::load(path)?;
        let [record] = document.records.as_slice() else {
            return Err(Error::new(
                "invalid_template",
                "A template must contain exactly one canonical decision",
            ));
        };
        Ok(record.clone())
    }
    /// Resolve the durable links stored in a record's Context field.
    /// # Errors
    /// Reports malformed or dangling references.
    pub fn relationships(&self, file: &str, id: u64) -> Result<Vec<Relationship>, Error> {
        let document = Document::load(self.path(file)?)?;
        let record = find(&document, id)?;
        let mut result = Vec::new();
        for event in Parser::new(&record.context) {
            if let Event::Start(Tag::Link { dest_url, .. }) = event
                && let Some(reference) = dest_url.strip_prefix("vrdx:")
            {
                let (path, id) = reference.rsplit_once('#').ok_or_else(|| {
                    Error::new("invalid_reference", "Missing decision identifier")
                })?;
                let file = decode(path)?;
                let id = id
                    .parse()
                    .map_err(|_| Error::new("invalid_reference", "Invalid decision identifier"))?;
                let target = Document::load(self.path(&file)?)?;
                let record = find(&target, id)?;
                result.push(Relationship {
                    file,
                    id,
                    title: record.title.clone(),
                });
            }
        }
        Ok(result)
    }
    /// Build a validated link for appending to a decision's Context field.
    /// # Errors
    /// Reports missing source or target decisions, or escaping paths.
    pub fn link(
        &self,
        file: &str,
        id: u64,
        target_file: &str,
        target_id: u64,
    ) -> Result<String, Error> {
        let source = Document::load(self.path(file)?)?;
        find(&source, id)?;
        let target = Document::load(self.path(target_file)?)?;
        find(&target, target_id)?;
        let target_file = self.relative(&target.path)?;
        Ok(format!(
            "[Decision {target_id}](vrdx:{}#{target_id})",
            encode(target_file.as_bytes())
        ))
    }
    /// List commits that changed a file, newest first.
    /// # Errors
    /// Reports unavailable Git, non-repositories, and command failures.
    pub fn history(&self, file: &str) -> Result<Vec<HistoryEntry>, Error> {
        Ok(self.history_page(file, 0, 200)?.entries)
    }
    /// Read a bounded history page; use the continuation offset for older commits.
    /// # Errors
    /// Rejects limits outside 1..=200 and reports Git failures.
    pub fn history_page(
        &self,
        file: &str,
        offset: usize,
        limit: usize,
    ) -> Result<HistoryPage, Error> {
        if !(1..=200).contains(&limit) {
            return Err(Error::new(
                "usage",
                "History limit must be between 1 and 200",
            ));
        }
        let file = self.relative(&self.path(file)?)?;
        let output = self.git(&[
            "log",
            &format!("--max-count={}", limit.saturating_add(1)),
            &format!("--skip={offset}"),
            "--format=%H%x09%s",
            "--",
            &file,
        ])?;
        let mut entries = output
            .lines()
            .filter_map(|line| line.split_once('\t'))
            .map(|(revision, summary)| HistoryEntry {
                revision: revision.into(),
                summary: summary.into(),
            })
            .collect::<Vec<_>>();
        let has_more = entries.len() > limit;
        entries.truncate(limit);
        let next_offset = offset.saturating_add(entries.len());
        Ok(HistoryPage {
            entries,
            has_more,
            next_offset,
        })
    }
    /// Read one decision from a verified historical commit without modifying disk.
    /// # Errors
    /// Reports invalid revisions, missing historical files, or missing decisions.
    pub fn historical(&self, file: &str, id: u64, revision: &str) -> Result<Record, Error> {
        if revision.is_empty() || revision.starts_with('-') || revision.contains(['\n', '\r', ':'])
        {
            return Err(Error::new("invalid_revision", "Invalid revision"));
        }
        let commit = self.git(&[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{revision}^{{commit}}"),
        ])?;
        let commit = commit.trim();
        if !commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(Error::new(
                "invalid_revision",
                "Git did not return a commit identifier",
            ));
        }
        let path = self.path(file)?;
        let file = self.relative(&path)?;
        // The Git toplevel may contain our selected root as a subdirectory.
        let prefix = self.git(&["rev-parse", "--show-prefix"])?;
        let source = self.git(&[
            "show",
            &format!(
                "{commit}:{}{file}",
                prefix.strip_suffix('\n').unwrap_or(&prefix)
            ),
        ])?;
        let document = Document::parse(path, source)?;
        Ok(find(&document, id)?.clone())
    }
    fn git(&self, args: &[&str]) -> Result<String, Error> {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .args(args)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GIT_LITERAL_PATHSPECS", "1")
            .env("GIT_PAGER", "cat")
            .env("GIT_TERMINAL_PROMPT", "0")
            .output()?;
        if !output.status.success() {
            return Err(Error::new(
                "git",
                String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            ));
        }
        String::from_utf8(output.stdout).map_err(|_| Error::new("git", "Git output was not UTF-8"))
    }
}

pub(crate) fn find(document: &Document, id: u64) -> Result<&Record, Error> {
    document
        .records
        .iter()
        .find(|record| record.id == id)
        .ok_or_else(|| Error::new("not_found", format!("Decision {id} does not exist")))
}
fn markdown(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("md") || extension.eq_ignore_ascii_case("markdown")
        })
}
fn discover(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.')
            || matches!(
                name.to_str(),
                Some(
                    "target"
                        | "node_modules"
                        | "vendor"
                        | "dist"
                        | "build"
                        | "venv"
                        | "__pycache__"
                )
            )
        {
            continue;
        }
        if kind.is_dir() {
            discover(&entry.path(), paths)?;
        } else if kind.is_file() && markdown(&entry.path()) {
            paths.push(entry.path());
        }
    }
    Ok(())
}
pub(crate) fn encode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'/') {
                char::from(*byte).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}
fn decode(value: &str) -> Result<String, Error> {
    let mut bytes = Vec::new();
    let mut input = value.bytes();
    while let Some(byte) = input.next() {
        if byte == b'%' {
            let high = input.next().and_then(|byte| char::from(byte).to_digit(16));
            let low = input.next().and_then(|byte| char::from(byte).to_digit(16));
            let (Some(high), Some(low)) = (high, low) else {
                return Err(Error::new("invalid_reference", "Invalid percent encoding"));
            };
            bytes.push(
                u8::try_from(high.saturating_mul(16).saturating_add(low))
                    .map_err(|_| Error::new("invalid_reference", "Invalid encoding"))?,
            );
        } else {
            bytes.push(byte);
        }
    }
    String::from_utf8(bytes)
        .map_err(|_| Error::new("invalid_reference", "Reference path must be UTF-8"))
}
