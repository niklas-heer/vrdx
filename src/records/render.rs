//! Human-readable rendering of the JSON data every command produces.

use serde_json::{Value, json};
use std::fmt::Write as _;

fn text(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned()
}

fn decision(value: &Value) -> String {
    let tags = value
        .get("tags")
        .and_then(Value::as_array)
        .map(|tags| {
            tags.iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    let mut output = format!(
        "{} [{}] {}\n  ID: {}\n  File: {}\n  Tags: {}\n",
        text(value, "date"),
        text(value, "status"),
        text(value, "title"),
        text(value, "id"),
        text(value, "file"),
        tags
    );
    if let Some(body) = value
        .get("body")
        .or_else(|| value.get("body_excerpt"))
        .and_then(Value::as_str)
    {
        output.push_str(body);
        output.push('\n');
    }
    if value.get("body_truncated") == Some(&Value::Bool(true)) {
        output.push_str("[Body truncated; use show ID for full text.]\n");
    }
    output
}

fn formatting(data: &Value, output: &mut String) {
    if let Some(files) = data.get("files").and_then(Value::as_array) {
        let check = data.get("check") == Some(&Value::Bool(true));
        if files.is_empty() {
            output.push_str("All records are formatted.\n");
        }
        for file in files.iter().filter_map(Value::as_str) {
            let _ = writeln!(
                output,
                "{}: {file}",
                if check {
                    "Needs formatting"
                } else {
                    "Formatted"
                }
            );
        }
        if check && !files.is_empty() {
            output.push_str("Run vrdx fmt with the same --dir to apply formatting.\n");
        }
    }
}

fn init(data: &Value, output: &mut String) {
    let Some(paths) = data.get("paths").and_then(Value::as_array) else {
        return;
    };
    let dry_run = data.get("dry_run") == Some(&Value::Bool(true));
    for path in paths {
        let _ = writeln!(output, "{:<9} {}", text(path, "status"), text(path, "path"));
    }
    let candidates: Vec<_> = data
        .pointer("/onboarding/candidates")
        .and_then(Value::as_array)
        .map(|list| list.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    if !candidates.is_empty() {
        let _ = writeln!(
            output,
            "Existing decision folders to import: {}. Ask your agent to onboard them.",
            candidates.join(", ")
        );
    }
    if data.pointer("/onboarding/collection_needs_import") == Some(&Value::Bool(true)) {
        let _ = writeln!(
            output,
            "{}/ contains Markdown that is not in vrdx format. Ask your agent to import it.",
            text(data, "collection")
        );
    }
    if dry_run {
        output.push_str("Dry run: nothing was written.\n");
    } else {
        let _ = writeln!(
            output,
            "Commit these files. Agents now consult and record decisions in {}/.",
            text(data, "collection")
        );
    }
}

pub(super) fn human(data: &Value) -> String {
    let mut output = String::new();
    if let Some(prompt) = data.get("prompt").and_then(Value::as_str) {
        return prompt.to_owned();
    }
    formatting(data, &mut output);
    init(data, &mut output);
    if let Some(guidance) = data.get("guidance").and_then(Value::as_str) {
        output.push_str(guidance);
        output.push_str("\n\n");
    }
    if let Some(record) = data.get("decision") {
        output.push_str(&decision(record));
    }
    if let Some(suggestions) = data.get("suggestions").and_then(Value::as_array) {
        if suggestions.is_empty() {
            output.push_str("No unlinked decisions share tags or significant words.\n");
        }
        for suggestion in suggestions {
            if let Some(record) = suggestion.get("decision") {
                output.push_str(&decision(record));
            }
            if let Some(reasons) = suggestion.get("reasons").and_then(Value::as_array) {
                for reason in reasons.iter().filter_map(Value::as_str) {
                    let _ = writeln!(output, "  {reason}");
                }
            }
        }
    }
    for key in ["decisions", "chain", "boundary_decisions"] {
        if let Some(records) = data.get(key).and_then(Value::as_array) {
            if records.is_empty() && key == "decisions" {
                output.push_str("No decisions matched.\n");
            }
            for record in records {
                output.push_str(&decision(record));
                output.push('\n');
            }
        }
    }
    if data.get("selection_truncated") == Some(&Value::Bool(true)) {
        output.push_str("[Selection limited; increase --limit for more matching decisions.]\n");
    }
    if let Some(relationships) = data.get("relationships").and_then(Value::as_array) {
        for relationship in relationships {
            if let Some(target) = relationship.get("decision") {
                let _ = writeln!(
                    output,
                    "{}: {} [{}] {}",
                    text(relationship, "relation"),
                    text(target, "id"),
                    text(target, "status"),
                    text(target, "title")
                );
            }
        }
        if relationships.is_empty() {
            output.push_str("No relationships.\n");
        }
    }
    if let Some(edges) = data.get("edges").and_then(Value::as_array) {
        for edge in edges {
            let _ = writeln!(
                output,
                "{} --{}--> {}",
                text(edge, "from"),
                text(edge, "relation"),
                text(edge, "to")
            );
        }
    }
    if let Some(valid) = data.get("valid").and_then(Value::as_bool) {
        output.push_str(if valid {
            "Collection is valid.\n"
        } else {
            "Collection is invalid.\n"
        });
    }
    if let Some(findings) = data.get("findings").and_then(Value::as_array) {
        for finding in findings {
            let _ = writeln!(
                output,
                "{}: {}: {}",
                text(finding, "file"),
                text(finding, "code"),
                text(finding, "message")
            );
            let _ = writeln!(output, "  Fix: {}", text(finding, "hint"));
        }
    }
    if let Some(graph) = data.get("graph") {
        if let Some(records) = graph.get("decisions").and_then(Value::as_object) {
            for record in records.values() {
                output.push_str(&decision(record));
            }
        }
        output.push_str(&human(
            &json!({"edges":graph.get("edges"),"findings":graph.get("findings")}),
        ));
    }
    output
}
