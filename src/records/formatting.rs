//! Conservative metadata formatting; Markdown is an opaque, preserved payload.

use super::{Error, Graph};
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};

fn canonical(source: &str) -> Result<String, Error> {
    const KEYS: [&str; 10] = [
        "schema_version",
        "id",
        "title",
        "date",
        "status",
        "tags",
        "supersedes",
        "superseded_by",
        "depends_on",
        "related_to",
    ];

    let mut lines = source.split_inclusive('\n');
    lines.next(); // The collection parser has already validated the delimiters.
    let mut header = String::new();
    for line in lines.by_ref() {
        if line.trim_end() == "+++" {
            break;
        }
        header.push_str(line);
    }
    let body: String = lines.collect();
    let mut document: toml_edit::DocumentMut = header
        .parse()
        .map_err(|error: toml_edit::TomlError| Error::new("invalid_record", error.to_string()))?;
    document.as_table_mut().sort_values_by(|a, _, b, _| {
        KEYS.iter()
            .position(|key| key == &a.get())
            .cmp(&KEYS.iter().position(|key| key == &b.get()))
    });
    for (mut key, item) in document.as_table_mut().iter_mut() {
        key.leaf_decor_mut().set_suffix(" ");
        if let Some(value) = item.as_value_mut() {
            value.decor_mut().set_prefix(" ");
        }
    }
    Ok(format!("+++\n{document}+++\n{body}"))
}

pub(super) fn format_collection(directory: &Path, check: bool) -> Result<(Value, bool), Error> {
    let graph = Graph::load(directory)?;
    graph.require_valid()?;
    let mut prepared = Vec::new();
    // Prepare and validate the entire collection before any replacement.
    for decision in graph.decisions.values() {
        let path = directory.join(&decision.file);
        let metadata = fs::symlink_metadata(&path)?;
        if !metadata.file_type().is_file() {
            return Err(Error::new("invalid_file", decision.file.clone()));
        }
        let original = fs::read_to_string(&path)?;
        let parsed = super::parse(decision.file.clone(), &original)?;
        if parsed.metadata != decision.metadata || parsed.body != decision.body {
            return Err(Error::new("concurrent_edit", decision.file.clone()));
        }
        let formatted = canonical(&original)?;
        let parsed = super::parse(decision.file.clone(), &formatted)?;
        if parsed.metadata != decision.metadata || parsed.body != decision.body {
            return Err(Error::new(
                "invalid_record",
                format!(
                    "Formatting would change data in {}; no files written.",
                    decision.file
                ),
            ));
        }
        if original != formatted {
            prepared.push((
                decision.file.clone(),
                path,
                original,
                formatted,
                metadata.permissions(),
            ));
        }
    }
    prepared.sort_by(|a, b| a.0.cmp(&b.0));
    let files: Vec<_> = prepared.iter().map(|entry| entry.0.clone()).collect();
    if !check {
        for (file, path, original, formatted, permissions) in prepared {
            let mut temp = tempfile::NamedTempFile::new_in(directory)?;
            temp.write_all(formatted.as_bytes())?;
            temp.as_file().set_permissions(permissions)?;
            temp.as_file().sync_all()?;
            if !fs::symlink_metadata(&path)?.file_type().is_file()
                || fs::read_to_string(&path)? != original
            {
                return Err(Error::new("concurrent_edit", file));
            }
            temp.persist(&path)
                .map_err(|error| Error::new("io", error.to_string()))?;
        }
    }
    let formatted = files.is_empty() || !check;
    Ok((
        json!({"check":check,"formatted":formatted,"files":files}),
        formatted,
    ))
}
