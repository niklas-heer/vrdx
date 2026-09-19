//! Observable collection workflows through the shipped binary.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Test fixtures fail immediately when setup or response shapes are wrong"
)]

use serde_json::{Value, json};
use std::{
    ffi::OsString,
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::TempDir;
use vrdx::records::{Metadata, Status};

const A: &str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
const B: &str = "01BRZ3NDEKTSV4RRFFQ69G5FAV";
const C: &str = "01CRZ3NDEKTSV4RRFFQ69G5FAV";
const D: &str = "01DRZ3NDEKTSV4RRFFQ69G5FAV";

fn binary() -> OsString {
    std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into())
}
fn raw(root: &Path, args: &[&str]) -> Output {
    Command::new(binary())
        .args(args)
        .current_dir(root)
        .output()
        .unwrap()
}
fn invoke(root: &Path, args: &[&str]) -> (i32, Value) {
    let mut values = vec!["--json", "--dir", "."];
    values.extend(args);
    let output = raw(root, &values);
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.stdout.contains(&27));
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["schema_version"], 1);
    (output.status.code().unwrap(), value)
}
fn success(root: &Path, args: &[&str]) -> Value {
    let (code, value) = invoke(root, args);
    assert_eq!(code, 0, "{value}");
    assert_eq!(value["ok"], true);
    value["data"].clone()
}
fn metadata(id: &str, status: Status) -> Metadata {
    Metadata {
        schema_version: 1,
        id: id.into(),
        title: format!("Decision {id}"),
        date: "2026-09-19".into(),
        status,
        tags: vec![],
        supersedes: vec![],
        superseded_by: vec![],
        depends_on: vec![],
        related_to: vec![],
    }
}
fn fixture(root: &Path, name: &str, metadata: &Metadata, body: &str) {
    fs::write(
        root.join(name),
        format!("+++\n{}+++\n{body}", toml::to_string(metadata).unwrap()),
    )
    .unwrap();
}
fn codes(value: &Value) -> Vec<&str> {
    value["data"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|finding| finding["code"].as_str().unwrap())
        .collect()
}

#[test]
fn create_rename_read_filter_and_search_preserve_body_and_identity() {
    let root = TempDir::new().unwrap();
    let body = "## Reasoning\r\n\r\n日本語 trade-off: simplicity over caching.\r\n\r\n## Consequences\r\nExtra latency.\r\n";
    fs::write(root.path().join("body.txt"), body).unwrap();
    let created = success(
        root.path(),
        &[
            "new",
            "Use \"Rust\" / 日本語",
            "--tag",
            "Platform tools",
            "--tag",
            "日本語",
            "--body-file",
            "body.txt",
            "--date",
            "2024-02-29",
            "--status",
            "accepted",
        ],
    );
    let id = created["decision"]["id"].as_str().unwrap();
    let file = created["decision"]["file"].as_str().unwrap();
    assert_eq!(id.len(), 26);
    assert!(!file.contains(id));
    assert!(file.starts_with("2024-02-29_"));
    assert!(file.ends_with("_use-rust.md"));
    let bytes = fs::read(root.path().join(file)).unwrap();
    fs::rename(root.path().join(file), root.path().join("renamed.md")).unwrap();
    let shown = success(root.path(), &["show", &id.to_lowercase()]);
    assert_eq!(shown["decision"]["id"], id);
    assert_eq!(shown["decision"]["body"], body);
    assert_eq!(shown["decision"]["file"], "renamed.md");
    assert_eq!(
        success(
            root.path(),
            &[
                "list",
                "--status",
                "accepted",
                "--tag",
                "platform TOOLS",
                "--tag",
                "日本語"
            ]
        )["decisions"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        success(root.path(), &["list", "--tag", "Platform"])["decisions"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    for (query, field) in [
        ("Rust", "title"),
        ("latency", "content"),
        ("日本語", "tags"),
        ("accepted", "status"),
        (id, "id"),
    ] {
        assert_eq!(
            success(root.path(), &["search", query, "--field", field])["decisions"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }
    assert_eq!(fs::read(root.path().join("renamed.md")).unwrap(), bytes);
    let human = raw(root.path(), &["--dir", ".", "show", id]);
    assert!(human.status.success());
    assert!(
        String::from_utf8(human.stdout)
            .unwrap()
            .contains("Extra latency.")
    );
}

#[test]
fn supersession_inverse_dependencies_and_context_are_explicit() {
    let root = TempDir::new().unwrap();
    let mut old = metadata(A, Status::Superseded);
    old.title = "Cache architecture".into();
    old.tags = vec!["performance".into()];
    old.superseded_by = vec![B.into()];
    fixture(
        root.path(),
        "z.md",
        &old,
        "## Consequences\nCaching caused stale reads.",
    );
    let mut middle = metadata(B, Status::Superseded);
    middle.supersedes = vec![A.into()];
    fixture(root.path(), "a.md", &middle, "Changed our approach.");
    let mut current = metadata(C, Status::Accepted);
    current.supersedes = vec![B.into()];
    current.depends_on = vec![D.into()];
    fixture(
        root.path(),
        "m.md",
        &current,
        "## Consequences\nFresh reads cost more.",
    );
    let mut dependency = metadata(D, Status::Deprecated);
    dependency.related_to = vec![C.into()];
    fixture(
        root.path(),
        "dependency.md",
        &dependency,
        "Historical service boundary.",
    );
    success(root.path(), &["validate"]);
    let first = success(root.path(), &["rebuild"]);
    assert_eq!(first, success(root.path(), &["rebuild"]));
    assert_eq!(first["graph"]["edges"].as_array().unwrap().len(), 4);
    let chain = success(root.path(), &["chain", "01AR"]);
    assert_eq!(
        chain["chain"]
            .as_array()
            .unwrap()
            .iter()
            .map(|record| record["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [A, B, C]
    );
    assert_eq!(chain["terminal"]["applies"], true);
    let relations = success(root.path(), &["relations", D]);
    assert!(
        relations["relationships"]
            .as_array()
            .unwrap()
            .iter()
            .any(|edge| edge["relation"] == "required_by")
    );
    let context = success(
        root.path(),
        &[
            "context",
            "cache",
            "--tag",
            "performance",
            "--limit",
            "1",
            "--body-chars",
            "10",
        ],
    );
    assert_eq!(context["matched_count"], 1);
    assert_eq!(context["decisions"].as_array().unwrap().len(), 3);
    assert_eq!(context["boundary_decisions"][0]["id"], D);
    assert_eq!(
        context["decisions"][0]["replacement_chain"],
        json!([A, B, C])
    );
    assert_eq!(context["decisions"][0]["body_truncated"], true);
    assert_eq!(context["decisions"][2]["applies"], true);
    assert_eq!(
        context,
        success(
            root.path(),
            &[
                "context",
                "cache",
                "--tag",
                "performance",
                "--limit",
                "1",
                "--body-chars",
                "10"
            ]
        )
    );
    fs::rename(root.path().join("z.md"), root.path().join("renamed.md")).unwrap();
    assert_eq!(success(root.path(), &["chain", A])["terminal"]["id"], C);
}

#[test]
fn human_context_reports_no_matches_only_when_no_decisions_match() {
    let root = TempDir::new().unwrap();
    fixture(
        root.path(),
        "accepted.md",
        &metadata(A, Status::Accepted),
        "A focused choice about caching.",
    );
    let matched = raw(root.path(), &["--dir", ".", "context", "caching"]);
    assert!(matched.status.success());
    assert!(matched.stderr.is_empty());
    let output = String::from_utf8(matched.stdout).unwrap();
    assert!(output.contains(A));
    assert!(output.contains("A focused choice about caching."));
    assert!(!output.contains("No decisions matched."));

    let unmatched = raw(root.path(), &["--dir", ".", "context", "unrelated"]);
    assert!(unmatched.status.success());
    assert!(unmatched.stderr.is_empty());
    let output = String::from_utf8(unmatched.stdout).unwrap();
    assert_eq!(output.matches("No decisions matched.").count(), 1);
    assert!(!output.contains(A));
}

#[test]
fn validation_aggregates_missing_self_duplicate_and_lifecycle_errors() {
    let root = TempDir::new().unwrap();
    let mut a = metadata(A, Status::Accepted);
    a.depends_on = vec![A.into(), D.into()];
    fixture(root.path(), "a.md", &a, "");
    fixture(root.path(), "duplicate.md", &a, "");
    let mut b = metadata(B, Status::Proposed);
    b.supersedes = vec![A.into()];
    fixture(root.path(), "b.md", &b, "");
    fixture(root.path(), "c.md", &metadata(C, Status::Superseded), "");
    fs::write(root.path().join("bad.md"), "not a record").unwrap();
    let (code, result) = invoke(root.path(), &["validate"]);
    assert_eq!(code, 1);
    for expected in [
        "duplicate_id",
        "self_reference",
        "missing_reference",
        "invalid_replacement",
        "status_mismatch",
        "missing_replacement",
        "invalid_record",
    ] {
        assert!(codes(&result).contains(&expected), "{result}");
    }
    assert_eq!(invoke(root.path(), &["rebuild"]).0, 1);
    let (_, error) = invoke(root.path(), &["context"]);
    assert_eq!(error["error"]["code"], "invalid_collection");
    assert!(!error["data"].is_object());
}

#[test]
fn cycles_and_conflicting_reciprocals_are_rejected() {
    let root = TempDir::new().unwrap();
    let mut a = metadata(A, Status::Superseded);
    a.superseded_by = vec![B.into()];
    let mut b = metadata(B, Status::Superseded);
    b.superseded_by = vec![A.into()];
    fixture(root.path(), "a.md", &a, "");
    fixture(root.path(), "b.md", &b, "");
    assert!(codes(&invoke(root.path(), &["validate"]).1).contains(&"supersession_cycle"));
    b.status = Status::Accepted;
    b.superseded_by.clear();
    fixture(root.path(), "b.md", &b, "");
    let mut c = metadata(C, Status::Accepted);
    c.supersedes = vec![A.into()];
    fixture(root.path(), "c.md", &c, "");
    assert!(codes(&invoke(root.path(), &["validate"]).1).contains(&"multiple_replacements"));
}

#[test]
fn metadata_errors_do_not_silently_disappear_or_write() {
    let root = TempDir::new().unwrap();
    let base = format!(
        "+++\n{}+++\nReasoning\n",
        toml::to_string(&metadata(A, Status::Accepted)).unwrap()
    );
    let variants = [
        base.replace("2026-09-19", "2025-02-29"),
        base.replace("accepted", "unknown"),
        base.replace("schema_version = 1", "schema_version = 2"),
        base.replace("tags = []", "tagz = []"),
        base.replace("tags = []", "tags = [\"Rust\", \"RUST\"]"),
        base.replace(A, "8ZZZZZZZZZZZZZZZZZZZZZZZZZ"),
        base.replace(A, &A.to_lowercase()),
        base.replace("tags = []", "tags = [\" \" ]"),
        base.replace("tags = []", "tags = []\ntags = []"),
        base.replace("depends_on = []", "depends_on = [\"01AR\"]"),
        base.replace(
            "depends_on = []",
            &format!("depends_on = [\"{B}\", \"{B}\"]"),
        ),
        base.trim_end_matches("Reasoning\n")
            .trim_end_matches("+++\n")
            .to_owned(),
    ];
    for source in variants {
        fs::write(root.path().join("invalid.md"), &source).unwrap();
        let (code, value) = invoke(root.path(), &["validate"]);
        assert_eq!(code, 1, "{source}\n{value}");
        assert!(codes(&value).contains(&"invalid_record"));
        assert_eq!(invoke(root.path(), &["new", "Should not write"]).0, 1);
        assert_eq!(
            fs::read_to_string(root.path().join("invalid.md")).unwrap(),
            source
        );
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    }
}

#[test]
fn json_errors_help_missing_directories_and_empty_collections() {
    let root = TempDir::new().unwrap();
    assert_eq!(success(root.path(), &["validate"])["records"], 0);
    assert_eq!(
        success(root.path(), &["list"])["decisions"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    for args in [
        vec!["nonsense"],
        vec!["search", ""],
        vec!["list", "--status", "invalid"],
        vec!["context", "--limit", "0"],
        vec!["context", "???"],
        vec!["show"],
    ] {
        assert_eq!(invoke(root.path(), &args).0, 2);
    }
    assert_eq!(invoke(root.path(), &["show", A]).0, 4);
    assert!(
        success(root.path(), &["--help"])["help"]
            .as_str()
            .unwrap()
            .contains("rebuild")
    );
    assert!(
        success(root.path(), &["--version"])["help"]
            .as_str()
            .unwrap()
            .contains("vrdx")
    );
    let result = raw(root.path(), &["--json", "--dir", "missing", "list"]);
    assert_eq!(result.status.code(), Some(4));
    let created = raw(
        root.path(),
        &["--json", "--dir", "missing", "new", "Create collection"],
    );
    assert!(created.status.success());
    fs::write(root.path().join("README.md"), "Collection instructions").unwrap();
    success(root.path(), &["validate"]);
}

#[test]
fn ambiguous_prefix_rejected_and_all_historical_statuses_are_non_applicable() {
    let root = TempDir::new().unwrap();
    for (id, status) in [
        (A, Status::Accepted),
        (B, Status::Rejected),
        (C, Status::Deprecated),
        (D, Status::Proposed),
    ] {
        fixture(
            root.path(),
            &format!("{id}.md"),
            &metadata(id, status),
            "Trade-offs.",
        );
    }
    assert_eq!(
        invoke(root.path(), &["show", "01"]).1["error"]["code"],
        "ambiguous_id"
    );
    let context = success(root.path(), &["context", "--limit", "2"]);
    assert_eq!(context["selection_truncated"], true);
    let all = success(root.path(), &["context"]);
    assert_eq!(
        all["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|record| record["applies"] == true)
            .count(),
        1
    );
    assert_eq!(
        success(root.path(), &["context", "nonexistentword"])["decisions"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
}

#[test]
fn creation_rejects_invalid_input_and_never_clobbers_colliding_names() {
    let root = TempDir::new().unwrap();
    let mut first = metadata(A, Status::Accepted);
    first.title = "Collision".into();
    let created = vrdx::records::create(root.path(), first.clone(), "Original body").unwrap();
    assert!(!created.file.contains(A));
    let before = fs::read(root.path().join(&created.file)).unwrap();
    fs::rename(
        root.path().join(&created.file),
        root.path().join("renamed.md"),
    )
    .unwrap();
    assert_eq!(
        vrdx::records::create(root.path(), first.clone(), "Overwrite")
            .unwrap_err()
            .code,
        "conflict"
    );
    fs::rename(
        root.path().join("renamed.md"),
        root.path().join(&created.file),
    )
    .unwrap();
    first.id = "01ARZ3NDEKTSV4RRFFQ69G5FAW".into();
    assert!(vrdx::records::create(root.path(), first, "Another body").is_err());
    assert_eq!(fs::read(root.path().join(created.file)).unwrap(), before);
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    for args in [
        vec!["new", "", "--date", "2026-09-19"],
        vec!["new", "Bad date", "--date", "2026-02-30"],
        vec!["new", "No replacement", "--status", "superseded"],
        vec!["new", "Bad tag", "--tag", " x"],
    ] {
        assert_ne!(invoke(root.path(), &args).0, 0);
    }
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn symlink_candidates_are_diagnosed_without_reading_external_records() {
    let root = TempDir::new().unwrap();
    let external = TempDir::new().unwrap();
    fixture(
        external.path(),
        "external.md",
        &metadata(A, Status::Accepted),
        "Outside",
    );
    std::os::unix::fs::symlink(
        external.path().join("external.md"),
        root.path().join("linked.md"),
    )
    .unwrap();
    let (_, value) = invoke(root.path(), &["validate"]);
    assert!(codes(&value).contains(&"invalid_file"));
    assert_eq!(value["data"]["records"], 0);
}

#[test]
fn long_history_rebuilds_after_every_rename_and_rejects_introduced_cycle() {
    let root = TempDir::new().unwrap();
    let ids: Vec<_> = (1_u128..=128)
        .map(|n| ulid::Ulid::from(n).to_string())
        .collect();
    for (index, id) in ids.iter().enumerate() {
        let mut record = metadata(
            id,
            if index == 127 {
                Status::Accepted
            } else {
                Status::Superseded
            },
        );
        record.superseded_by = ids
            .get(index.saturating_add(1))
            .cloned()
            .into_iter()
            .collect();
        fixture(
            root.path(),
            &format!("{index:03}.md"),
            &record,
            "A documented consequence.",
        );
    }
    let original = success(root.path(), &["chain", &ids[0]]);
    assert_eq!(original["chain"].as_array().unwrap().len(), 128);
    for index in [0, 127, 64, 32] {
        fs::rename(
            root.path().join(format!("{index:03}.md")),
            root.path().join(format!("renamed-{index}.md")),
        )
        .unwrap();
        assert_eq!(
            success(root.path(), &["chain", &ids[0]])["terminal"]["id"],
            ids[127]
        );
    }
    let mut last = metadata(&ids[127], Status::Superseded);
    last.superseded_by = vec![ids[0].clone()];
    fixture(root.path(), "renamed-127.md", &last, "Introduced cycle");
    assert!(codes(&invoke(root.path(), &["validate"]).1).contains(&"supersession_cycle"));
    last.status = Status::Accepted;
    last.superseded_by.clear();
    fixture(root.path(), "renamed-127.md", &last, "Repaired");
    success(root.path(), &["validate"]);
}
