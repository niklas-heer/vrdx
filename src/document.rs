//! Source-preserving decision parsing and conflict-aware atomic persistence.

use std::{
    collections::BTreeSet,
    fmt, fs,
    io::Write,
    ops::Range,
    path::{Path, PathBuf},
};

use patterns::{
    record::{Field, Fielded},
    seq::{Keyed, found_ref},
};
use pulldown_cmark::{Event, Parser, Tag};

const START: &str = "<!-- vrdx start -->";
const END: &str = "<!-- vrdx end -->";
const FIELDS: [&str; 4] = ["Status", "Decision", "Context", "Consequences"];

/// An editable decision. Narrative fields may be empty, including for drafts.
///
/// The premise named-field representation preserves Unicode and every `u64`
/// identifier. Identifiers use decimal text rather than premise's signed
/// whole-number representation.
///
/// ```
/// use patterns::record::Fielded;
/// use vrdx::document::Record;
///
/// let mut decision = Record::new(u64::MAX);
/// decision.title = "Use Rust — café 日本語".into();
/// let stored = decision.field();
/// assert_eq!(Record::refielded(&stored), Some(decision));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u64,
    pub title: String,
    pub status: String,
    pub decision: String,
    pub context: String,
    pub consequences: String,
}

impl Keyed for Record {
    type Key = u64;

    fn key(&self) -> Self::Key {
        self.id
    }
}

impl Fielded for Record {
    fn field(&self) -> Field {
        Field::Table(vec![
            ("id".into(), Field::Text(self.id.to_string())),
            ("title".into(), Field::Text(self.title.clone())),
            ("status".into(), Field::Text(self.status.clone())),
            ("decision".into(), Field::Text(self.decision.clone())),
            ("context".into(), Field::Text(self.context.clone())),
            (
                "consequences".into(),
                Field::Text(self.consequences.clone()),
            ),
        ])
    }

    fn refielded(field: &Field) -> Option<Self> {
        let Field::Table(rows) = field else {
            return None;
        };
        let expected = [
            "id",
            "title",
            "status",
            "decision",
            "context",
            "consequences",
        ];
        let mut seen = BTreeSet::new();
        if rows.len() != expected.len()
            || rows
                .iter()
                .any(|(key, _)| !expected.contains(&key.as_str()) || !seen.insert(key.as_str()))
        {
            return None;
        }
        let text = |key| {
            rows.iter()
                .find(|(name, _)| name == key)
                .and_then(|(_, value)| match value {
                    Field::Text(value) => Some(value.as_str()),
                    _ => None,
                })
        };
        let id = text("id")?;
        if id.is_empty() || !id.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        Some(Self {
            id: id.parse().ok()?,
            title: text("title")?.into(),
            status: text("status")?.into(),
            decision: text("decision")?.into(),
            context: text("context")?.into(),
            consequences: text("consequences")?.into(),
        })
    }
}

patterns::rationale::because!(
    Record,
    "Decision identity and named fields use premise contracts; decimal text preserves the full u64 range that premise's signed whole-number representation cannot hold"
);

impl Record {
    /// Construct an empty draft using the supplied identifier.
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            title: String::new(),
            status: "📝 Draft".into(),
            decision: String::new(),
            context: String::new(),
            consequences: String::new(),
        }
    }

    fn render(&self, newline: &str) -> String {
        let fields = [
            &self.status,
            &self.decision,
            &self.context,
            &self.consequences,
        ];
        let mut output = format!("### {} {}", self.id, self.title);
        for (label, value) in FIELDS.iter().zip(fields) {
            output.push_str(newline);
            output.push_str("* **");
            output.push_str(label);
            output.push_str("**: ");
            output.push_str(&value.replace('\n', newline));
        }
        output
    }
}

/// Failures which callers can distinguish from a successful save.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Invalid(String),
    Conflict(PathBuf),
    IdOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::Invalid(message) => write!(f, "Invalid decision document: {message}"),
            Self::Conflict(path) => write!(
                f,
                "File changed on disk: {}; reload before saving (your draft is retained)",
                path.display()
            ),
            Self::IdOverflow => f.write_str("No decision identifier remains after u64::MAX"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// A document and the exact disk snapshot against which edits are checked.
#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub records: Vec<Record>,
    pub has_markers: bool,
    source: String,
    baseline: Option<Vec<u8>>,
    spans: Vec<Range<usize>>,
    block: Option<Range<usize>>,
}

impl Document {
    /// Read a UTF-8 document without translating its newlines or BOM.
    ///
    /// # Errors
    /// Returns an I/O error or a diagnostic for malformed decision data.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        // Resolve symlinks once: replacement must target the file, not its link.
        let path = fs::canonicalize(path)?;
        let source = fs::read_to_string(&path)?;
        Self::parse(path, source)
    }

    /// Prepare a new, absent file. Saving never overwrites a pre-existing path.
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self {
            path,
            records: Vec::new(),
            has_markers: false,
            source: String::new(),
            baseline: None,
            spans: Vec::new(),
            block: None,
        }
    }

    /// Parse a source snapshot, retaining untouched source spans.
    ///
    /// # Errors
    /// Rejects malformed markers, fields, identifiers, or ambiguous records.
    pub fn parse(path: PathBuf, source: String) -> Result<Self, Error> {
        let block = marker_block(&source)?;
        let (records, spans) = match &block {
            Some(range) => parse_records(&source, range.clone())?,
            None => (Vec::new(), Vec::new()),
        };
        Ok(Self {
            path,
            records,
            has_markers: block.is_some(),
            baseline: Some(source.as_bytes().to_vec()),
            source,
            spans,
            block,
        })
    }

    /// Return the next unused identifier, starting at zero.
    ///
    /// # Errors
    /// Returns [`Error::IdOverflow`] when the largest identifier cannot increase.
    pub fn next_id(&self) -> Result<u64, Error> {
        self.records
            .iter()
            .map(|record| record.id)
            .max()
            .map_or(Ok(0), |id| id.checked_add(1).ok_or(Error::IdOverflow))
    }

    /// Save one edited or new record; leave all unrelated bytes unchanged.
    ///
    /// The whole-file baseline is checked twice. An uncooperative writer can
    /// still race the final check and rename: this is not compare-and-swap.
    /// Temporary contents are synced before replacement; directory metadata
    /// durability across power loss is not promised.
    ///
    /// # Errors
    /// Returns validation, conflict, or I/O failures without changing this
    /// document's snapshot. Failed saves leave the caller's record untouched.
    pub fn save_record(&mut self, record: &Record) -> Result<(), Error> {
        self.save_with_hook(record, || Ok(()))
    }

    fn save_with_hook(
        &mut self,
        record: &Record,
        before_replace: impl FnOnce() -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.check_baseline()?;
        if found_ref(&self.records, record.id) == Some(record) {
            return Ok(());
        }
        validate_record(record)?;
        let updated = self.edited_source(record)?;
        let parsed = Self::parse(self.path.clone(), updated)?;
        if found_ref(&parsed.records, record.id).map(Fielded::field) != Some(record.field()) {
            return Err(Error::Invalid("The edit cannot round-trip without changing its fields; check surrounding whitespace and Markdown field syntax".into()));
        }
        let parent = self
            .path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        if self.baseline.is_some() {
            temporary
                .as_file()
                .set_permissions(fs::metadata(&self.path)?.permissions())?;
        }
        temporary.write_all(parsed.source.as_bytes())?;
        temporary.flush()?;
        temporary.as_file().sync_all()?;
        before_replace()?;
        self.check_baseline()?;
        let result = if self.baseline.is_some() {
            temporary.persist(&self.path)
        } else {
            temporary.persist_noclobber(&self.path)
        };
        result.map_err(|failure| {
            if failure.error.kind() == std::io::ErrorKind::AlreadyExists {
                Error::Conflict(self.path.clone())
            } else {
                Error::Io(failure.error)
            }
        })?;
        *self = parsed;
        Ok(())
    }

    fn check_baseline(&self) -> Result<(), Error> {
        match (&self.baseline, fs::read(&self.path)) {
            (Some(expected), Ok(actual)) if *expected == actual => Ok(()),
            (None, Err(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                // A dangling symlink is an existing path, too.
                match fs::symlink_metadata(&self.path) {
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
                    Err(error) => Err(Error::Io(error)),
                    Ok(_) => Err(Error::Conflict(self.path.clone())),
                }
            }
            (_, Err(error)) if error.kind() != std::io::ErrorKind::NotFound => {
                Err(Error::Io(error))
            }
            _ => Err(Error::Conflict(self.path.clone())),
        }
    }

    fn edited_source(&self, record: &Record) -> Result<String, Error> {
        let newline = if self.source.contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let rendered = record.render(newline);
        if let Some(index) = self
            .records
            .iter()
            .position(|existing| existing.id == record.id)
        {
            let span = self
                .spans
                .get(index)
                .ok_or_else(|| Error::Invalid("Missing original record span".into()))?;
            return splice(&self.source, span.clone(), &rendered);
        }
        if let Some(block) = &self.block {
            let position = self.spans.first().map_or(block.start, |span| span.start);
            let inserted = if self.spans.is_empty() {
                format!("{newline}{rendered}{newline}")
            } else {
                format!("{rendered}{newline}{newline}")
            };
            return splice(&self.source, position..position, &inserted);
        }
        let separator = if self.source.is_empty() || self.source.ends_with('\n') {
            ""
        } else {
            newline
        };
        Ok(format!(
            "{}{separator}{START}{newline}{rendered}{newline}{END}{newline}",
            self.source
        ))
    }
}

fn validate_record(record: &Record) -> Result<(), Error> {
    if record.title.trim().is_empty()
        || record.status.trim().is_empty()
        || record.title.contains(['\r', '\n'])
        || record.status.contains(['\r', '\n'])
    {
        return Err(Error::Invalid(
            "Title and status must be nonempty single lines".into(),
        ));
    }
    Ok(())
}

fn splice(source: &str, range: Range<usize>, replacement: &str) -> Result<String, Error> {
    Ok(format!(
        "{}{replacement}{}",
        slice(source, 0..range.start)?,
        slice(source, range.end..source.len())?
    ))
}

fn slice(source: &str, range: Range<usize>) -> Result<&str, Error> {
    source
        .get(range)
        .ok_or_else(|| Error::Invalid("Invalid source boundary".into()))
}

fn code_ranges(source: &str) -> Vec<Range<usize>> {
    Parser::new(source)
        .into_offset_iter()
        .filter_map(|(event, range)| match event {
            Event::Start(Tag::CodeBlock(_)) | Event::Code(_) => Some(range),
            _ => None,
        })
        .collect()
}

fn marker_block(source: &str) -> Result<Option<Range<usize>>, Error> {
    let excluded = code_ranges(source);
    let positions = |marker: &str| {
        source
            .match_indices(marker)
            .map(|(position, _)| position)
            .filter(|position| !excluded.iter().any(|range| range.contains(position)))
            .collect::<Vec<_>>()
    };
    let starts = positions(START);
    let ends = positions(END);
    if starts.is_empty() && ends.is_empty() {
        return Ok(None);
    }
    match (starts.as_slice(), ends.as_slice()) {
        ([start], [end]) if start < end => Ok(Some(start.saturating_add(START.len())..*end)),
        ([_], [_]) => Err(Error::Invalid("End marker precedes start marker".into())),
        _ => Err(Error::Invalid(
            "Expected exactly one start and one end marker outside code examples".into(),
        )),
    }
}

fn lines_with_offsets(source: &str) -> impl Iterator<Item = (usize, &str)> {
    source.split_inclusive('\n').scan(0_usize, |offset, line| {
        let start = *offset;
        *offset = offset.saturating_add(line.len());
        Some((start, line.trim_end_matches(['\r', '\n'])))
    })
}

fn heading(line: &str) -> Result<Option<(u64, String)>, Error> {
    let Some(rest) = line.strip_prefix("### ") else {
        return Ok(None);
    };
    let Some((id, title)) = rest.trim_start().split_once(char::is_whitespace) else {
        return Ok(None);
    };
    let id = id.strip_suffix('.').unwrap_or(id);
    if !id.bytes().all(|byte| byte.is_ascii_digit()) || id.is_empty() {
        return Ok(None);
    }
    let id = id
        .parse::<u64>()
        .map_err(|_| Error::Invalid("Decision identifier exceeds u64::MAX".into()))?;
    Ok(Some((id, title.trim().to_owned())))
}

fn parse_records(
    source: &str,
    block: Range<usize>,
) -> Result<(Vec<Record>, Vec<Range<usize>>), Error> {
    let body = slice(source, block.clone())?;
    let excluded = code_ranges(body);
    let mut headings = Vec::new();
    let mut ids = BTreeSet::new();
    for (offset, line) in lines_with_offsets(body) {
        if excluded.iter().any(|range| range.contains(&offset)) {
            continue;
        }
        if let Some((id, title)) = heading(line)? {
            if !ids.insert(id) {
                return Err(Error::Invalid(format!(
                    "Duplicate decision identifier {id}"
                )));
            }
            headings.push((offset, id, title));
        }
    }
    let mut records = Vec::new();
    let mut spans = Vec::new();
    for (index, (start, id, title)) in headings.iter().enumerate() {
        let end = headings
            .get(index.saturating_add(1))
            .map_or(body.len(), |(offset, _, _)| *offset);
        let section = slice(body, *start..end)?.trim_end();
        let fields = parse_fields(section, *id)?;
        let mut shape = vec![
            ("id".into(), Field::Text(id.to_string())),
            ("title".into(), Field::Text(title.clone())),
        ];
        shape.extend(
            FIELDS
                .into_iter()
                .zip(fields)
                .map(|(name, value)| (name.to_lowercase(), Field::Text(value))),
        );
        let record = Record::refielded(&Field::Table(shape)).ok_or_else(|| {
            Error::Invalid(format!("Decision {id} has an invalid named-field shape"))
        })?;
        validate_record(&record)?;
        records.push(record);
        let absolute = block.start.saturating_add(*start);
        spans.push(absolute..absolute.saturating_add(section.len()));
    }
    if records.is_empty() && !body.trim().is_empty() {
        return Err(Error::Invalid(
            "Marker block contains no valid decisions".into(),
        ));
    }
    Ok((records, spans))
}

fn parse_fields(section: &str, id: u64) -> Result<Vec<String>, Error> {
    let excluded = code_ranges(section);
    let mut found = Vec::new();
    for (offset, line) in lines_with_offsets(section).skip(1) {
        if excluded.iter().any(|range| range.contains(&offset)) {
            continue;
        }
        for label in FIELDS {
            let prefix = format!("* **{label}**:");
            if let Some(value) = line.strip_prefix(&prefix) {
                let value = value.strip_prefix(' ').unwrap_or(value);
                let value_start = offset.saturating_add(line.len().saturating_sub(value.len()));
                found.push((label, offset, value_start));
            }
        }
    }
    if let Some((_, first_field, _)) = found.first() {
        let heading_end = section
            .find('\n')
            .map_or(section.len(), |offset| offset.saturating_add(1));
        if !slice(section, heading_end..*first_field)?.trim().is_empty() {
            return Err(Error::Invalid(format!(
                "Decision {id}: unexpected content before its first field"
            )));
        }
    }
    let mut values = Vec::new();
    for label in FIELDS {
        let matching: Vec<_> = found
            .iter()
            .enumerate()
            .filter(|(_, (name, _, _))| *name == label)
            .collect();
        let [(index, (_, _, value_start))] = matching.as_slice() else {
            return Err(Error::Invalid(format!(
                "Decision {id}: expected exactly one {label} field"
            )));
        };
        let value_end = found
            .get(index.saturating_add(1))
            .map_or(section.len(), |(_, offset, _)| *offset);
        values.push(
            slice(section, *value_start..value_end)?
                .trim_end()
                .replace("\r\n", "\n"),
        );
    }
    Ok(values)
}

patterns::rationale::because!(
    Document,
    "Retain exact source spans and a raw disk baseline so a semantic edit changes only its record and a stale save refuses to overwrite concurrent work"
);

#[cfg(test)]
mod tests;
