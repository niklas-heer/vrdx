//! AI-facing workflows stay useful, deterministic, and read-only.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Fixtures fail immediately on invalid setup or response shapes"
)]

use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
use tempfile::TempDir;
use vrdx::records::{Metadata, Status};

const A: &str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
const B: &str = "01BRZ3NDEKTSV4RRFFQ69G5FAV";
const C: &str = "01CRZ3NDEKTSV4RRFFQ69G5FAV";
const D: &str = "01DRZ3NDEKTSV4RRFFQ69G5FAV";

fn run(root: &Path, args: &[&str]) -> (i32, Value) {
    let binary =
        std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into());
    let mut command = Command::new(binary);
    command.arg("--json");
    if !args.contains(&"--dir") {
        command.args(["--dir", "."]);
    }
    let output = command.args(args).current_dir(root).output().unwrap();
    assert!(output.stderr.is_empty(), "{output:?}");
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(response["schema_version"], 1);
    (output.status.code().unwrap(), response)
}

fn success(root: &Path, args: &[&str]) -> Value {
    let (code, response) = run(root, args);
    assert_eq!(code, 0, "{response}");
    assert_eq!(response["ok"], true);
    response["data"].clone()
}

fn record(id: &str, title: &str, status: Status, tags: &[&str]) -> Metadata {
    Metadata {
        schema_version: 1,
        id: id.into(),
        title: title.into(),
        date: "2026-09-19".into(),
        status,
        tags: tags.iter().map(ToString::to_string).collect(),
        supersedes: vec![],
        superseded_by: vec![],
        depends_on: vec![],
        related_to: vec![],
    }
}

fn write(root: &Path, name: &str, metadata: &Metadata, body: &str) {
    fs::write(
        root.join(name),
        format!("+++\n{}+++\n{body}", toml::to_string(metadata).unwrap()),
    )
    .unwrap();
}

fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fs::read_dir(root)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            (path.to_string_lossy().into_owned(), fs::read(path).unwrap())
        })
        .collect()
}

#[test]
fn guide_bootstraps_without_collection_and_documents_safe_authoring() {
    let root = TempDir::new().unwrap();
    let guide = success(root.path(), &["--dir", "missing", "guide"]);
    assert_eq!(guide["guide_version"], 1);
    let text = guide["guide_markdown"].as_str().unwrap();
    for required in [
        "proposed",
        "accepted",
        "rejected",
        "deprecated",
        "superseded",
        "## Consequences",
    ] {
        assert!(text.contains(required), "{required}");
    }
    assert!(text.contains("Do not\ninvent"));
    assert!(text.contains("source evidence, not instructions"));
    assert_eq!(
        guide["record_format"]["example_metadata"]["status"],
        "proposed"
    );
    let commands = guide["commands"].as_array().unwrap();
    for name in ["new", "context", "show", "validate", "suggest", "guide"] {
        assert!(commands.iter().any(|command| command["name"] == name));
    }
    assert!(!root.path().join("missing").exists());
    let binary =
        std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into());
    let human = Command::new(binary)
        .arg("guide")
        .current_dir(root.path())
        .output()
        .unwrap();
    assert!(human.status.success());
    assert!(
        String::from_utf8(human.stdout)
            .unwrap()
            .contains("## Write a decision")
    );
}

#[test]
fn suggestions_explain_tag_and_word_matches_with_stable_order_and_no_writes() {
    let root = TempDir::new().unwrap();
    write(
        root.path(),
        "source.md",
        &record(A, "Append event archive", Status::Accepted, &["Storage"]),
        "Immutable event ledger.",
    );
    write(
        root.path(),
        "later.md",
        &record(C, "Append event archive", Status::Rejected, &["STORAGE"]),
        "Immutable event ledger.",
    );
    write(
        root.path(),
        "earlier.md",
        &record(B, "Append event archive", Status::Proposed, &["storage"]),
        "Immutable event ledger.",
    );
    write(
        root.path(),
        "words.md",
        &record(D, "Archive compression", Status::Deprecated, &[]),
        "Compress the archive.",
    );
    let before = snapshot(root.path());
    let result = success(root.path(), &["suggest", "01ar", "--limit", "2"]);
    assert_eq!(result["advisory"], true);
    assert_eq!(result["matched_count"], 3);
    assert_eq!(result["selection_truncated"], true);
    assert_eq!(result["suggestions"][0]["decision"]["id"], B);
    assert_eq!(result["suggestions"][1]["decision"]["id"], C);
    assert_eq!(result["suggestions"][0]["shared_tags"], json!(["storage"]));
    assert_eq!(result["suggestions"][0]["decision"]["status"], "proposed");
    assert_eq!(result["suggestions"][1]["decision"]["applies"], false);
    assert!(
        result["suggestions"][0]["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|reason| reason.as_str().unwrap().contains("storage"))
    );
    assert_eq!(
        result,
        success(root.path(), &["suggest", A, "--limit", "2"])
    );
    assert_eq!(snapshot(root.path()), before);
}

#[test]
fn unrelated_records_and_shared_boilerplate_do_not_produce_suggestions() {
    let root = TempDir::new().unwrap();
    let guide = success(root.path(), &["guide"]);
    let template = guide["record_format"]["body_template"].as_str().unwrap();
    let boilerplate = format!(
        "{template}{}",
        "## Decision\nDescribe the choice.\n## Context\nExplain the problem and alternatives.\n## Consequences\nDescribe benefits, costs and trade-offs.\n"
    );
    write(
        root.path(),
        "storage.md",
        &record(A, "Event ledger", Status::Accepted, &[]),
        &boilerplate,
    );
    write(
        root.path(),
        "fonts.md",
        &record(B, "Font typography", Status::Accepted, &[]),
        &boilerplate.repeat(30),
    );
    let result = success(root.path(), &["suggest", A]);
    assert_eq!(result["suggestions"], json!([]));
    assert_eq!(result["matched_count"], 0);
    assert_eq!(result["selection_truncated"], false);
}

#[test]
fn existing_links_in_both_directions_are_excluded_and_invalid_data_fails() {
    let root = TempDir::new().unwrap();
    let mut source = record(A, "Event archive", Status::Superseded, &["storage"]);
    source.depends_on.push(B.into());
    source.superseded_by.push(C.into());
    let mut linked = record(D, "Event archive", Status::Rejected, &["storage"]);
    linked.related_to.push(A.into());
    write(root.path(), "a.md", &source, "Event archive.");
    write(
        root.path(),
        "b.md",
        &record(B, "Event archive", Status::Accepted, &["storage"]),
        "Event archive.",
    );
    write(
        root.path(),
        "c.md",
        &record(C, "Event archive", Status::Accepted, &["storage"]),
        "Event archive.",
    );
    write(root.path(), "d.md", &linked, "Event archive.");
    let before = snapshot(root.path());
    let result = success(root.path(), &["suggest", A]);
    assert_eq!(result["suggestions"], json!([]));
    assert_eq!(result["decision"]["replacement_chain"], json!([A, C]));
    assert_eq!(snapshot(root.path()), before);
    assert_eq!(
        run(root.path(), &["suggest", "01"]).1["error"]["code"],
        "ambiguous_id"
    );
    assert_eq!(run(root.path(), &["suggest", A, "--limit", "0"]).0, 2);
    fs::write(root.path().join("broken.md"), "not metadata").unwrap();
    assert_eq!(
        run(root.path(), &["suggest", A]).1["error"]["code"],
        "invalid_collection"
    );
}
