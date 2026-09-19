//! Standalone Markdown collections. Disk is authoritative; graphs are disposable.

mod ai;
pub mod cli;
mod dashboard;

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
};
use ulid::Ulid;

/// A stable error code and a human-readable explanation.
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
    pub(crate) fn exit_code(&self) -> u8 {
        match self.code {
            "usage" => 2,
            "conflict" => 3,
            "not_found" => 4,
            _ => 1,
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Proposed,
    Accepted,
    Rejected,
    Deprecated,
    Superseded,
}
impl Status {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Deprecated => "deprecated",
            Self::Superseded => "superseded",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub schema_version: u8,
    pub id: String,
    pub title: String,
    pub date: String,
    pub status: Status,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub supersedes: Vec<String>,
    #[serde(default)]
    pub superseded_by: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub related_to: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Decision {
    #[serde(flatten)]
    pub metadata: Metadata,
    pub file: String,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Supersedes,
    DependsOn,
    RelatedTo,
}
impl Relation {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Supersedes => "supersedes",
            Self::DependsOn => "depends_on",
            Self::RelatedTo => "related_to",
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    pub from: String,
    pub relation: Relation,
    pub to: String,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    pub file: String,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Default, Serialize)]
pub struct Graph {
    pub decisions: BTreeMap<String, Decision>,
    pub edges: BTreeSet<Edge>,
    pub findings: Vec<Finding>,
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new("invalid_record", message)
}

fn valid_id(value: &str) -> bool {
    value
        .parse::<Ulid>()
        .is_ok_and(|id| id.to_string() == value && !id.is_nil())
}

fn validate_metadata(metadata: &Metadata) -> Result<(), Error> {
    if metadata.schema_version != 1 {
        return Err(invalid("Unsupported schema_version; expected 1"));
    }
    if !valid_id(&metadata.id) {
        return Err(invalid("id must be a nonzero canonical uppercase ULID"));
    }
    if metadata.title.trim().is_empty() || metadata.title.chars().any(char::is_control) {
        return Err(invalid("title must be nonempty and on one line"));
    }
    let date = metadata
        .date
        .parse::<jiff::civil::Date>()
        .map_err(|error| invalid(error.to_string()))?;
    if metadata.date.len() != 10 || date.to_string() != metadata.date {
        return Err(invalid("date must use YYYY-MM-DD"));
    }
    let mut tags = BTreeSet::new();
    for tag in &metadata.tags {
        if tag.trim().is_empty() || tag.trim() != tag || tag.chars().any(char::is_control) {
            return Err(invalid(
                "tags must be nonempty, trimmed, single-line strings",
            ));
        }
        if !tags.insert(tag.to_lowercase()) {
            return Err(invalid(format!("Duplicate tag: {tag}")));
        }
    }
    for values in [
        &metadata.supersedes,
        &metadata.superseded_by,
        &metadata.depends_on,
        &metadata.related_to,
    ] {
        let mut seen = BTreeSet::new();
        for value in values {
            if !valid_id(value) {
                return Err(invalid(format!(
                    "Relationship target must be a full uppercase ULID: {value}"
                )));
            }
            if !seen.insert(value) {
                return Err(invalid(format!("Duplicate relationship target: {value}")));
            }
        }
    }
    Ok(())
}

fn parse(file: String, source: &str) -> Result<Decision, Error> {
    let mut lines = source.split_inclusive('\n');
    if lines.next().map(str::trim_end) != Some("+++") {
        return Err(invalid(
            "Expected +++ TOML metadata at the start of the file",
        ));
    }
    let mut metadata = String::new();
    let mut closed = false;
    for line in lines.by_ref() {
        if line.trim_end() == "+++" {
            closed = true;
            break;
        }
        metadata.push_str(line);
    }
    if !closed {
        return Err(invalid("Missing closing +++ metadata delimiter"));
    }
    let metadata: Metadata =
        toml::from_str(&metadata).map_err(|error| invalid(error.to_string()))?;
    validate_metadata(&metadata)?;
    Ok(Decision {
        metadata,
        file,
        body: lines.collect(),
    })
}

impl Graph {
    /// Read a flat collection, collecting document errors without skipping them silently.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be enumerated.
    pub fn load(directory: &Path) -> Result<Self, Error> {
        let mut paths = fs::read_dir(directory)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()?;
        paths.sort();
        let mut graph = Self::default();
        for path in paths {
            if path
                .extension()
                .is_none_or(|ext| !ext.eq_ignore_ascii_case("md"))
                || path
                    .file_name()
                    .is_some_and(|name| name.eq_ignore_ascii_case("README.md"))
            {
                continue;
            }
            let file = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| invalid("Decision filenames must be UTF-8"))?
                .to_owned();
            if !fs::symlink_metadata(&path)?.file_type().is_file() {
                graph.report(
                    &file,
                    "invalid_file",
                    "Decision files must be regular files, not directories or symlinks",
                );
                continue;
            }
            let result = fs::read_to_string(&path)
                .map_err(Error::from)
                .and_then(|source| parse(file.clone(), &source));
            match result {
                Ok(decision) => {
                    let id = decision.metadata.id.clone();
                    if let Some(previous) = graph.decisions.get(&id) {
                        graph.report(
                            &file,
                            "duplicate_id",
                            format!("{id} also occurs in {}", previous.file),
                        );
                    } else {
                        graph.decisions.insert(id, decision);
                    }
                }
                Err(error) => graph.report(&file, error.code, error.message),
            }
        }
        graph.build_edges();
        graph.validate_edges();
        graph.findings.sort();
        graph.findings.dedup();
        Ok(graph)
    }

    fn report(&mut self, file: &str, code: &'static str, message: impl Into<String>) {
        self.findings.push(Finding {
            file: file.into(),
            code,
            message: message.into(),
        });
    }

    fn build_edges(&mut self) {
        for decision in self.decisions.values() {
            let record = &decision.metadata;
            for (relation, targets, inverse) in [
                (Relation::Supersedes, &record.supersedes, false),
                (Relation::Supersedes, &record.superseded_by, true),
                (Relation::DependsOn, &record.depends_on, false),
                (Relation::RelatedTo, &record.related_to, false),
            ] {
                for target in targets {
                    let (mut from, mut to) = if inverse {
                        (target.clone(), record.id.clone())
                    } else {
                        (record.id.clone(), target.clone())
                    };
                    if relation == Relation::RelatedTo && from > to {
                        std::mem::swap(&mut from, &mut to);
                    }
                    self.edges.insert(Edge {
                        from,
                        relation: relation.clone(),
                        to,
                    });
                }
            }
        }
    }

    fn validate_edges(&mut self) {
        let mut findings = Vec::new();
        for edge in &self.edges {
            let source = self.decisions.get(&edge.from);
            let target = self.decisions.get(&edge.to);
            let file = source
                .or(target)
                .map_or("", |decision| decision.file.as_str());
            let mut report = |code, message| {
                findings.push(Finding {
                    file: file.into(),
                    code,
                    message,
                });
            };
            if edge.from == edge.to {
                report("self_reference", format!("{} refers to itself", edge.from));
            }
            if source.is_none() || target.is_none() {
                report(
                    "missing_reference",
                    format!(
                        "Unresolved edge {} {} {}",
                        edge.from,
                        edge.relation.label(),
                        edge.to
                    ),
                );
            }
            if edge.relation == Relation::Supersedes {
                if let Some(source) = source
                    && matches!(source.metadata.status, Status::Proposed | Status::Rejected)
                {
                    report(
                        "invalid_replacement",
                        format!(
                            "{} cannot supersede another decision while {}",
                            edge.from,
                            source.metadata.status.label()
                        ),
                    );
                }
                if let Some(target) = target
                    && target.metadata.status != Status::Superseded
                {
                    report(
                        "status_mismatch",
                        format!("{} has a replacement but is not superseded", edge.to),
                    );
                }
            }
        }
        let successors = self.successors();
        for (id, decision) in &self.decisions {
            let replacements = successors.get(id).map_or(0, BTreeSet::len);
            if replacements > 1 {
                findings.push(Finding {
                    file: decision.file.clone(),
                    code: "multiple_replacements",
                    message: format!("{id} has more than one replacement"),
                });
            }
            if replacements == 0 && decision.metadata.status == Status::Superseded {
                findings.push(Finding {
                    file: decision.file.clone(),
                    code: "missing_replacement",
                    message: format!("{id} is superseded but has no replacement"),
                });
            }
        }
        self.findings.extend(findings);
        self.validate_cycles();
    }

    fn validate_cycles(&mut self) {
        // Kahn's algorithm avoids recursion even for large, hand-edited graphs.
        let mut incoming: BTreeMap<_, usize> =
            self.decisions.keys().map(|id| (id.clone(), 0)).collect();
        let mut outgoing: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for edge in &self.edges {
            if edge.relation == Relation::Supersedes
                && self.decisions.contains_key(&edge.from)
                && self.decisions.contains_key(&edge.to)
            {
                if let Some(count) = incoming.get_mut(&edge.to) {
                    *count = count.saturating_add(1);
                }
                outgoing
                    .entry(edge.from.clone())
                    .or_default()
                    .push(edge.to.clone());
            }
        }
        let mut ready: BTreeSet<String> = incoming
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(id, _)| id.clone())
            .collect();
        while let Some(id) = ready.pop_first() {
            for target in outgoing.get(&id).into_iter().flatten() {
                if let Some(count) = incoming.get_mut(target) {
                    *count = count.saturating_sub(1);
                    if *count == 0 {
                        ready.insert(target.clone());
                    }
                }
            }
            incoming.remove(&id);
        }
        if !incoming.is_empty() {
            self.findings.push(Finding {
                file: String::new(),
                code: "supersession_cycle",
                message: format!(
                    "Cycle blocks these decisions (including downstream records): {}",
                    incoming.keys().cloned().collect::<Vec<_>>().join(", ")
                ),
            });
        }
    }

    pub(crate) fn successors(&self) -> BTreeMap<String, BTreeSet<String>> {
        let mut result: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for edge in &self.edges {
            if edge.relation == Relation::Supersedes {
                result
                    .entry(edge.to.clone())
                    .or_default()
                    .insert(edge.from.clone());
            }
        }
        result
    }

    pub(crate) fn require_valid(&self) -> Result<(), Error> {
        if self.findings.is_empty() {
            Ok(())
        } else {
            Err(Error::new(
                "invalid_collection",
                format!(
                    "{} finding(s); run vrdx validate for details",
                    self.findings.len()
                ),
            ))
        }
    }

    pub(crate) fn resolve(&self, query: &str) -> Result<&Decision, Error> {
        let query = query.to_uppercase();
        if query.is_empty() {
            return Err(Error::new("usage", "ID prefix cannot be empty"));
        }
        let mut matches = self
            .decisions
            .values()
            .filter(|decision| decision.metadata.id.starts_with(&query));
        let decision = matches
            .next()
            .ok_or_else(|| Error::new("not_found", format!("No decision matches {query}")))?;
        if matches.next().is_some() {
            return Err(Error::new(
                "ambiguous_id",
                format!("ID prefix {query} matches multiple decisions"),
            ));
        }
        Ok(decision)
    }

    pub(crate) fn chain(&self, id: &str) -> Vec<String> {
        let successors = self.successors();
        let mut result = Vec::new();
        let mut seen = BTreeSet::new();
        let mut current = Some(id.to_owned());
        while let Some(id) = current {
            if !seen.insert(id.clone()) {
                break;
            }
            current = successors
                .get(&id)
                .and_then(|values| values.first())
                .cloned();
            result.push(id);
        }
        result
    }
}

/// Publish a fully prepared new decision without replacing any existing file.
///
/// # Errors
/// Returns metadata, filesystem, or publication errors. Existing files are never edited.
pub fn create(directory: &Path, metadata: Metadata, body: &str) -> Result<Decision, Error> {
    validate_metadata(&metadata)?;
    if directory.exists() {
        let graph = Graph::load(directory)?;
        graph.require_valid()?;
        if graph.decisions.contains_key(&metadata.id) {
            return Err(Error::new("conflict", "Identity already exists"));
        }
    }
    if metadata.status == Status::Superseded
        || !metadata.supersedes.is_empty()
        || !metadata.superseded_by.is_empty()
        || !metadata.depends_on.is_empty()
        || !metadata.related_to.is_empty()
    {
        return Err(invalid(
            "Create an independent record, then edit relationships and lifecycle in Markdown",
        ));
    }
    let slug: String = metadata
        .title
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .chars()
        .take(60)
        .collect();
    let slug = if slug.is_empty() {
        "decision"
    } else {
        slug.trim_end_matches('-')
    };
    let id = metadata
        .id
        .parse::<Ulid>()
        .map_err(|error| invalid(error.to_string()))?;
    let millis = i64::try_from(id.timestamp_ms()).map_err(|error| invalid(error.to_string()))?;
    let timestamp =
        jiff::Timestamp::from_millisecond(millis).map_err(|error| invalid(error.to_string()))?;
    let file = format!(
        "{}_{}_{}.md",
        metadata.date,
        timestamp.strftime("%H%M%S%3f"),
        slug
    );
    let header = toml::to_string(&metadata).map_err(|error| invalid(error.to_string()))?;
    let source = format!("+++\n{header}+++\n{body}");
    fs::create_dir_all(directory)?;
    let mut temp = tempfile::NamedTempFile::new_in(directory)?;
    temp.write_all(source.as_bytes())?;
    temp.as_file().sync_all()?;
    temp.persist_noclobber(directory.join(&file))
        .map_err(|error| Error::new("io", error.to_string()))?;
    Ok(Decision {
        metadata,
        file,
        body: body.into(),
    })
}
