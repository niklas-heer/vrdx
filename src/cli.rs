//! Deterministic JSON commands for agents and scripts; no terminal required.

pub use crate::repository::Error;
use crate::{
    document::{Document, Record},
    repository::{Repository, find},
};
use patterns::record::{Field, Fielded};
use serde::de::{Deserialize, Deserializer, Error as _, MapAccess, Visitor};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fmt;
use std::{collections::BTreeMap, fs};

/// Machine interface usage, also available through `vrdx agent help`.
#[must_use]
pub const fn help() -> &'static str {
    "vrdx agent COMMAND [--root DIRECTORY] [OPTIONS]\nCommands: list, show, search, create, update, delete, move, merge, validate, templates, template, relationships, link, follow, history, history-show\nAll output is JSON. IDs are decimal strings. Use --file ROOT_RELATIVE_MARKDOWN --id ID.\nshow without --id returns the complete document snapshot. Mutations require --if-match SNAPSHOT returned by show/list (use missing for a new file).\ncreate/update: --record JSON (title/status/decision/context/consequences; optional id on create), or --template NAME on create.\nLarge inputs: --record-file JSON_FILE and --baseline-file MARKDOWN_FILE replace inline options. search: --query TEXT. move: --position ZERO_BASED_INDEX. merge: --baseline-source ORIGINAL_MARKDOWN --record JSON.\nlink/follow: --target-file FILE --target-id ID. template: --name NAME. history: --offset N --limit N (1..200), follow next_offset while has_more. history-show: --revision COMMIT."
}

/// Execute one headless command, returning a JSON success envelope.
/// # Errors
/// Reports usage, validation, path, Git, and conflict errors with stable codes.
pub fn run(args: &[String]) -> Result<Value, Error> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(Error::new("usage", help()));
    };
    if matches!(command, "help" | "--help" | "-h") {
        if args.len() != 1 {
            return Err(Error::new("usage", "Help accepts no options"));
        }
        return Ok(json!({"ok":true,"help":help(),"schema_version":1}));
    }
    let options = Options::parse(args.get(1..).unwrap_or_default())?;
    options.validate(command)?;
    let repository = Repository::new(options.value("root").unwrap_or("."))?;
    let data = match command {
        "list" | "search" => listing(&repository, &options, command == "search")?,
        "show" => show(
            &repository,
            options.required("file")?,
            options.value("id").map(|_| options.id("id")).transpose()?,
        )?,
        "create" | "update" | "delete" | "move" | "merge" | "link" => {
            mutate(&repository, &options, command)?
        }
        "validate" => validate(&repository)?,
        "templates" => json!({"templates":repository.templates()?}),
        "template" => {
            json!({"record":record_json(&repository.template(options.required("name")?)?)})
        }
        "relationships" => {
            let values = repository
                .relationships(options.required("file")?, options.id("id")?)?
                .into_iter()
                .map(|link| json!({"file":link.file,"id":link.id.to_string(),"title":link.title}))
                .collect::<Vec<_>>();
            json!({"relationships":values})
        }
        "follow" => {
            let file = options.required("file")?;
            let id = options.id("id")?;
            let target_file = options.required("target-file")?;
            let target_id = options.id("target-id")?;
            let links = repository.relationships(file, id)?;
            if !links
                .iter()
                .any(|link| link.file == target_file && link.id == target_id)
            {
                return Err(Error::new(
                    "not_found",
                    "The source decision has no such relationship",
                ));
            }
            show(&repository, target_file, Some(target_id))?
        }
        "history" => {
            let offset = options.number("offset", 0)?;
            let page = repository.history_page(
                options.required("file")?,
                offset,
                options.number("limit", 200)?,
            )?;
            json!({"history":page.entries.into_iter().map(|entry|json!({"revision":entry.revision,"summary":entry.summary})).collect::<Vec<_>>(),"offset":offset,"has_more":page.has_more,"next_offset":page.next_offset})
        }
        "history-show" => {
            json!({"record":record_json(&repository.historical(options.required("file")?,options.id("id")?,options.required("revision")?)?)})
        }
        _ => return Err(Error::new("usage", format!("Unknown command {command}"))),
    };
    Ok(json!({"ok":true,"schema_version":1,"data":data}))
}

struct Options(BTreeMap<String, String>);
impl Options {
    fn parse(args: &[String]) -> Result<Self, Error> {
        let mut options = BTreeMap::new();
        let mut values = args.iter();
        while let Some(name) = values.next() {
            let name = name
                .strip_prefix("--")
                .filter(|name| !name.is_empty())
                .ok_or_else(|| Error::new("usage", "Options must use --name VALUE pairs"))?;
            let value = values
                .next()
                .ok_or_else(|| Error::new("usage", format!("Missing value for --{name}")))?;
            if options.insert(name.into(), value.clone()).is_some() {
                return Err(Error::new("usage", format!("Duplicate option --{name}")));
            }
        }
        Ok(Self(options))
    }
    fn value(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }
    fn required(&self, key: &str) -> Result<&str, Error> {
        self.value(key)
            .ok_or_else(|| Error::new("usage", format!("Missing --{key}")))
    }
    fn id(&self, key: &str) -> Result<u64, Error> {
        let value = self.required(key)?;
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::new(
                "usage",
                format!("--{key} must be a decimal u64"),
            ));
        }
        value
            .parse()
            .map_err(|_| Error::new("usage", format!("--{key} exceeds u64")))
    }
    fn input(&self, inline: &str, file: &str) -> Result<String, Error> {
        match (self.value(inline), self.value(file)) {
            (Some(value), None) => Ok(value.into()),
            (None, Some(path)) => Ok(fs::read_to_string(path)?),
            (Some(_), Some(_)) => Err(Error::new(
                "usage",
                format!("Choose --{inline} or --{file}"),
            )),
            (None, None) => Err(Error::new(
                "usage",
                format!("Missing --{inline} or --{file}"),
            )),
        }
    }
    fn number(&self, key: &str, default: usize) -> Result<usize, Error> {
        self.value(key).map_or(Ok(default), |value| {
            value
                .parse()
                .map_err(|_| Error::new("usage", format!("--{key} must be a nonnegative integer")))
        })
    }
    fn validate(&self, command: &str) -> Result<(), Error> {
        let allowed: &[&str] = match command {
            "list" | "validate" | "templates" => &[],
            "search" => &["query"],
            "show" | "relationships" => &["file", "id"],
            "create" => &["file", "record", "record-file", "template", "if-match"],
            "update" => &["file", "id", "record", "record-file", "if-match"],
            "delete" => &["file", "id", "if-match"],
            "move" => &["file", "id", "position", "if-match"],
            "merge" => &[
                "file",
                "id",
                "record",
                "record-file",
                "baseline-source",
                "baseline-file",
                "if-match",
            ],
            "link" => &["file", "id", "target-file", "target-id", "if-match"],
            "follow" => &["file", "id", "target-file", "target-id"],
            "template" => &["name"],
            "history" => &["file", "offset", "limit"],
            "history-show" => &["file", "id", "revision"],
            _ => return Err(Error::new("usage", format!("Unknown command {command}"))),
        };
        for key in self.0.keys() {
            if key != "root" && !allowed.contains(&key.as_str()) {
                return Err(Error::new(
                    "usage",
                    format!("Unknown option --{key} for {command}"),
                ));
            }
        }
        Ok(())
    }
}

fn listing(repository: &Repository, options: &Options, search: bool) -> Result<Value, Error> {
    let query = if search {
        Some(options.required("query")?.to_lowercase())
    } else {
        None
    };
    let mut records = Vec::new();
    for document in repository.documents()? {
        let file = repository.relative(&document.path)?;
        let snapshot = snapshot(document.source().as_bytes());
        for record in &document.records {
            if query.as_ref().is_none_or(|query| {
                [
                    &record.title,
                    &record.status,
                    &record.decision,
                    &record.context,
                    &record.consequences,
                ]
                .iter()
                .any(|text| text.to_lowercase().contains(query))
            }) {
                records.push(json!({"file":file,"snapshot":snapshot,"record":record_json(record)}));
            }
        }
    }
    Ok(json!({"records":records}))
}
fn show(repository: &Repository, file: &str, id: Option<u64>) -> Result<Value, Error> {
    let path = repository.path(file)?;
    let source = fs::read_to_string(&path)?;
    let document = Document::parse(path, source.clone())?;
    let mut result = json!({"file":repository.relative(&document.path)?,"snapshot":snapshot(source.as_bytes()),"source":source,"records":document.records.iter().map(record_json).collect::<Vec<_>>()});
    if let Some(id) = id {
        result
            .as_object_mut()
            .ok_or_else(|| Error::new("invalid_json", "Invalid response shape"))?
            .insert("record".into(), record_json(find(&document, id)?));
    }
    Ok(result)
}
fn validate(repository: &Repository) -> Result<Value, Error> {
    let documents = repository.documents()?;
    let mut count = 0_usize;
    for document in &documents {
        let file = repository.relative(&document.path)?;
        for record in &document.records {
            repository.relationships(&file, record.id)?;
            count = count.saturating_add(1);
        }
    }
    for name in repository.templates()? {
        repository.template(&name)?;
    }
    Ok(json!({"valid":true,"documents":documents.len(),"records":count}))
}
fn mutate(repository: &Repository, options: &Options, command: &str) -> Result<Value, Error> {
    let file = options.required("file")?;
    let path = repository.path(file)?;
    let source = match fs::read_to_string(&path) {
        Ok(source) => Some(source),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    let expected = options.required("if-match")?;
    let actual = source
        .as_ref()
        .map_or_else(|| "missing".into(), |source| snapshot(source.as_bytes()));
    if actual != expected {
        return Err(Error::new(
            "conflict",
            "Snapshot does not match the current file; reread before retrying",
        ));
    }
    let mut document = source.map_or_else(
        || Ok(Document::new(path.clone())),
        |source| Document::parse(path.clone(), source),
    )?;
    let id = if command == "create" {
        document.next_id()?
    } else {
        let id = options.id("id")?;
        find(&document, id)?;
        id
    };
    match command {
        "create" => {
            if (options.value("record").is_some() || options.value("record-file").is_some())
                && options.value("template").is_some()
            {
                return Err(Error::new(
                    "usage",
                    "Choose --record or --template, not both",
                ));
            }
            let record = if let Some(name) = options.value("template") {
                let mut record = repository.template(name)?;
                record.id = id;
                record
            } else {
                parse_record(&options.input("record", "record-file")?, &Record::new(id))?
            };
            document.save_record(&record)?;
        }
        "update" => {
            let record = parse_record(
                &options.input("record", "record-file")?,
                find(&document, id)?,
            )?;
            document.save_record(&record)?;
        }
        "delete" => document.delete_record(id)?,
        "move" => {
            let position = options
                .required("position")?
                .parse::<usize>()
                .map_err(|_| Error::new("usage", "Position must be a nonnegative index"))?;
            document.move_record(id, position)?;
        }
        "merge" => {
            let baseline = options.input("baseline-source", "baseline-file")?;
            let mut original = Document::parse(path, baseline)?;
            let record = parse_record(
                &options.input("record", "record-file")?,
                find(&original, id)?,
            )?;
            original.merge_record_against(&record, document)?;
            document = original;
        }
        "link" => {
            let link = repository.link(
                file,
                id,
                options.required("target-file")?,
                options.id("target-id")?,
            )?;
            let mut record = find(&document, id)?.clone();
            if !record.context.contains(&link) {
                if !record.context.is_empty() {
                    record.context.push_str("\n\n");
                }
                record.context.push_str(&link);
            }
            document.save_record(&record)?;
        }
        _ => return Err(Error::new("usage", "Unsupported mutation")),
    }
    Ok(
        json!({"file":repository.relative(&document.path)?,"snapshot":snapshot(document.source().as_bytes()),"id":id.to_string(),"records":document.records.iter().map(record_json).collect::<Vec<_>>()}),
    )
}

fn snapshot(source: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(source))
}
fn record_json(record: &Record) -> Value {
    let Field::Table(fields) = record.field() else {
        return Value::Null;
    };
    Value::Object(
        fields
            .into_iter()
            .filter_map(|(name, field)| match field {
                Field::Text(text) => Some((name, Value::String(text))),
                _ => None,
            })
            .collect(),
    )
}
fn parse_record(source: &str, base: &Record) -> Result<Record, Error> {
    let UniqueRecord(patch) = serde_json::from_str(source)
        .map_err(|error| Error::new("invalid_json", error.to_string()))?;
    let Value::Object(mut shape) = record_json(base) else {
        return Err(Error::new("invalid_json", "Invalid Premise record shape"));
    };
    for (name, value) in patch {
        if !shape.contains_key(&name) || !value.is_string() {
            return Err(Error::new(
                "invalid_json",
                format!("Unknown field or non-string value: {name}"),
            ));
        }
        shape.insert(name, value);
    }
    let fields = shape
        .into_iter()
        .map(|(name, value)| match value {
            Value::String(text) => Ok((name, Field::Text(text))),
            _ => Err(Error::new("invalid_json", "Record fields must be strings")),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let record = Record::refielded(&Field::Table(fields))
        .ok_or_else(|| Error::new("invalid_json", "Record violates Premise Fielded schema"))?;
    if record.id != base.id {
        return Err(Error::new(
            "invalid_json",
            "Record identity cannot be changed",
        ));
    }
    Ok(record)
}

// Serde's generic Value accepts repeated object keys. Agent mutations require an
// unambiguous field patch, so reject duplicates before constructing the shape.
struct UniqueRecord(BTreeMap<String, Value>);
impl<'de> Deserialize<'de> for UniqueRecord {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RecordVisitor;
        impl<'de> Visitor<'de> for RecordVisitor {
            type Value = UniqueRecord;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an object with unique decision field names")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut fields = BTreeMap::new();
                while let Some((name, value)) = map.next_entry::<String, Value>()? {
                    if fields.insert(name.clone(), value).is_some() {
                        return Err(M::Error::custom(format!("Duplicate field: {name}")));
                    }
                }
                Ok(UniqueRecord(fields))
            }
        }
        deserializer.deserialize_map(RecordVisitor)
    }
}
