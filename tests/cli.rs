//! Exercise the public agent interface through a real subprocess without a TTY.

use serde_json::{Value, json};
use std::{ffi::OsString, fs, path::Path, process::Command};
use tempfile::TempDir;

fn binary() -> OsString {
    std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into())
}
#[expect(
    clippy::expect_used,
    reason = "Process fixture failures should report the setup operation"
)]
fn invoke(root: &Path, args: &[&str]) -> (i32, Value) {
    let output = Command::new(binary())
        .arg("agent")
        .args(args)
        .current_dir(root)
        .output()
        .expect("launch CLI");
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !output.stdout.contains(&27),
        "headless output contains terminal escapes"
    );
    let value = serde_json::from_slice(&output.stdout).expect("one valid JSON response");
    (output.status.code().expect("normal process exit"), value)
}
#[expect(
    clippy::expect_used,
    reason = "Fixture success helpers require a successful structured response"
)]
fn success(root: &Path, args: &[&str]) -> Value {
    let (status, value) = invoke(root, args);
    assert_eq!(status, 0, "{value}");
    assert_eq!(value.get("ok"), Some(&Value::Bool(true)));
    value.get("data").expect("data envelope").clone()
}
fn create(root: &Path, file: &str, title: &str, snapshot: &str) -> Value {
    success(
        root,
        &[
            "create",
            "--file",
            file,
            "--if-match",
            snapshot,
            "--record",
            &json!({"title":title}).to_string(),
        ],
    )
}
#[expect(
    clippy::expect_used,
    reason = "Fixtures inspect the documented response schema"
)]
fn token(value: &Value) -> &str {
    value
        .get("snapshot")
        .and_then(Value::as_str)
        .expect("snapshot token")
}
#[expect(
    clippy::expect_used,
    reason = "Git fixture operations must succeed before testing history"
)]
fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .expect("run fixture git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("UTF-8 Git output")
}

#[test]
fn complete_crud_search_ordering_and_stale_agent_rejection() {
    let root = TempDir::new().unwrap();
    let first = create(root.path(), "README.md", "Use 日本語 Rust", "missing");
    assert_eq!(first["id"], "0");
    let second = create(root.path(), "README.md", "Second decision", token(&first));
    let list = success(root.path(), &["list"]);
    assert_eq!(list["records"].as_array().unwrap().len(), 2);
    assert_eq!(list["records"][0]["record"]["id"], "1");
    let found = success(root.path(), &["search", "--query", "日本語"]);
    assert_eq!(found["records"][0]["record"]["id"], "0");
    let before = fs::read(root.path().join("README.md")).unwrap();
    let (status, error) = invoke(
        root.path(),
        &[
            "update",
            "--file",
            "README.md",
            "--id",
            "0",
            "--if-match",
            token(&first),
            "--record",
            r#"{"title":"stale"}"#,
        ],
    );
    assert_eq!(status, 3);
    assert_eq!(error["error"]["code"], "conflict");
    assert_eq!(fs::read(root.path().join("README.md")).unwrap(), before);
    let updated = success(
        root.path(),
        &[
            "update",
            "--file",
            "README.md",
            "--id",
            "0",
            "--if-match",
            token(&second),
            "--record",
            r#"{"decision":"Preserve newlines.\n\nAnd Unicode é."}"#,
        ],
    );
    let moved = success(
        root.path(),
        &[
            "move",
            "--file",
            "README.md",
            "--id",
            "0",
            "--position",
            "0",
            "--if-match",
            token(&updated),
        ],
    );
    assert_eq!(moved["records"][0]["id"], "0");
    let deleted = success(
        root.path(),
        &[
            "delete",
            "--file",
            "README.md",
            "--id",
            "1",
            "--if-match",
            token(&moved),
        ],
    );
    assert_eq!(deleted["records"].as_array().unwrap().len(), 1);
    let third = create(
        root.path(),
        "README.md",
        "Monotonic identity",
        token(&deleted),
    );
    assert_eq!(third["id"], "2");
    assert_eq!(success(root.path(), &["validate"])["valid"], true);
}

#[test]
fn markerless_documents_can_be_inspected_and_initialized_without_losing_prose() {
    let root = TempDir::new().unwrap();
    fs::write(root.path().join("notes.markdown"), "# Existing notes\n").unwrap();
    let document = success(root.path(), &["show", "--file", "notes.markdown"]);
    assert_eq!(document["records"], json!([]));
    create(root.path(), "notes.markdown", "Added", token(&document));
    assert!(
        fs::read_to_string(root.path().join("notes.markdown"))
            .unwrap()
            .starts_with("# Existing notes\n")
    );
}

#[test]
fn strict_arguments_json_and_required_preconditions_never_write() {
    let root = TempDir::new().unwrap();
    let created = create(root.path(), "README.md", "Original", "missing");
    let original = fs::read(root.path().join("README.md")).unwrap();
    for args in [
        vec!["wat"],
        vec!["list", "--wat", "x"],
        vec!["list", "--root", ".", "--root", "."],
        vec![
            "update",
            "--file",
            "README.md",
            "--id",
            "0",
            "--record",
            r#"{"title":"No precondition"}"#,
        ],
    ] {
        assert_eq!(invoke(root.path(), &args).0, 2);
    }
    for patch in [
        "{",
        r#"{"unknown":"x"}"#,
        r#"{"title":"first","title":"second"}"#,
        r#"{"id":0}"#,
        r#"{"id":"1"}"#,
        r#"{"title":""}"#,
    ] {
        let (status, error) = invoke(
            root.path(),
            &[
                "update",
                "--file",
                "README.md",
                "--id",
                "0",
                "--if-match",
                token(&created),
                "--record",
                patch,
            ],
        );
        assert_ne!(status, 0, "{error}");
    }
    assert_eq!(fs::read(root.path().join("README.md")).unwrap(), original);
}

#[test]
fn templates_and_durable_links_roundtrip_reserved_filename_characters() {
    let root = TempDir::new().unwrap();
    fs::create_dir_all(root.path().join(".vrdx/templates")).unwrap();
    let seed = create(root.path(), "seed.md", "Template title", "missing");
    let updated = success(
        root.path(),
        &[
            "update",
            "--file",
            "seed.md",
            "--id",
            "0",
            "--if-match",
            token(&seed),
            "--record",
            r#"{"decision":"Repeatable narrative"}"#,
        ],
    );
    assert_ne!(token(&updated), "");
    fs::copy(
        root.path().join("seed.md"),
        root.path().join(".vrdx/templates/design.md"),
    )
    .unwrap();
    assert_eq!(
        success(root.path(), &["templates"])["templates"],
        json!(["design"])
    );
    assert_eq!(
        success(root.path(), &["template", "--name", "design"])["record"]["decision"],
        "Repeatable narrative"
    );
    let source = success(
        root.path(),
        &[
            "create",
            "--file",
            "source.md",
            "--template",
            "design",
            "--if-match",
            "missing",
        ],
    );
    let target_file = "café :[a]#%().md";
    create(root.path(), target_file, "Target", "missing");
    let linked = success(
        root.path(),
        &[
            "link",
            "--file",
            "source.md",
            "--id",
            "0",
            "--target-file",
            target_file,
            "--target-id",
            "0",
            "--if-match",
            token(&source),
        ],
    );
    assert!(
        linked["records"][0]["context"]
            .as_str()
            .unwrap()
            .contains("%23%25%28%29")
    );
    let links = success(
        root.path(),
        &["relationships", "--file", "source.md", "--id", "0"],
    );
    assert_eq!(links["relationships"][0]["file"], target_file);
    assert_eq!(
        success(
            root.path(),
            &[
                "follow",
                "--file",
                "source.md",
                "--id",
                "0",
                "--target-file",
                target_file,
                "--target-id",
                "0"
            ]
        )["record"]["title"],
        "Target"
    );
    assert_eq!(success(root.path(), &["validate"])["valid"], true);
    fs::remove_file(root.path().join(target_file)).unwrap();
    assert_ne!(invoke(root.path(), &["validate"]).0, 0);
}

#[test]
fn invalid_templates_and_external_paths_are_rejected() {
    let root = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    fs::create_dir_all(root.path().join(".vrdx/templates")).unwrap();
    fs::write(root.path().join(".vrdx/templates/empty.md"), "No decisions").unwrap();
    assert_ne!(invoke(root.path(), &["template", "--name", "empty"]).0, 0);
    assert_ne!(
        invoke(root.path(), &["template", "--name", "../../outside"]).0,
        0
    );
    for file in [
        "../outside.md",
        "/tmp/outside.md",
        "./local.md",
        "notes.txt",
    ] {
        let (_, error) = invoke(
            root.path(),
            &[
                "create",
                "--file",
                file,
                "--record",
                r#"{"title":"No"}"#,
                "--if-match",
                "missing",
            ],
        );
        assert_eq!(error["error"]["code"], "invalid_path");
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(outside.path(), root.path().join("escape")).unwrap();
        let (_, error) = invoke(
            root.path(),
            &[
                "create",
                "--file",
                "escape/outside.md",
                "--record",
                r#"{"title":"No"}"#,
                "--if-match",
                "missing",
            ],
        );
        assert_eq!(error["error"]["code"], "invalid_path");
        assert!(!outside.path().join("outside.md").exists());
    }
}

#[test]
fn history_is_read_only_literal_and_handles_nested_repository_roots() {
    let root = TempDir::new().unwrap();
    git(root.path(), &["init", "--quiet"]);
    git(root.path(), &["config", "user.name", "Test"]);
    git(
        root.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    fs::create_dir(root.path().join(" space")).unwrap();
    let nested = root.path().join(" space");
    let file = ":[literal]*.md";
    create(&nested, file, "Historical title", "missing");
    git(root.path(), &["add", "--all"]);
    git(
        root.path(),
        &["commit", "--quiet", "-m", "feat: first record"],
    );
    let first = git(root.path(), &["rev-parse", "HEAD"]).trim().to_owned();
    let current = success(&nested, &["show", "--file", file, "--id", "0"]);
    success(
        &nested,
        &[
            "update",
            "--file",
            file,
            "--id",
            "0",
            "--if-match",
            token(&current),
            "--record",
            r#"{"title":"Current title"}"#,
        ],
    );
    git(root.path(), &["add", "--all"]);
    git(
        root.path(),
        &["commit", "--quiet", "-m", "fix: change title"],
    );
    let before = fs::read(nested.join(file)).unwrap();
    let history = success(&nested, &["history", "--file", file]);
    assert_eq!(history["history"].as_array().unwrap().len(), 2);
    let page = success(&nested, &["history", "--file", file, "--limit", "1"]);
    assert_eq!(page["has_more"], true);
    assert_eq!(page["next_offset"], 1);
    let older = success(
        &nested,
        &["history", "--file", file, "--limit", "1", "--offset", "1"],
    );
    assert_eq!(older["has_more"], false);
    assert_eq!(older["history"][0]["revision"], first);
    assert_eq!(
        invoke(&nested, &["history", "--file", file, "--limit", "0"]).0,
        2
    );
    let old = success(
        &nested,
        &[
            "history-show",
            "--file",
            file,
            "--id",
            "0",
            "--revision",
            &first,
        ],
    );
    assert_eq!(old["record"]["title"], "Historical title");
    assert_eq!(fs::read(nested.join(file)).unwrap(), before);
    assert_ne!(
        invoke(
            &nested,
            &[
                "history-show",
                "--file",
                file,
                "--id",
                "0",
                "--revision",
                "--output=/tmp/evil"
            ]
        )
        .0,
        0
    );
    assert_eq!(git(root.path(), &["status", "--porcelain"]), "");
}

#[test]
fn three_way_merge_accepts_independent_changes_and_rejects_competing_edits() {
    let root = TempDir::new().unwrap();
    create(root.path(), "README.md", "Original", "missing");
    let baseline = success(root.path(), &["show", "--file", "README.md", "--id", "0"]);
    let remote = success(
        root.path(),
        &[
            "update",
            "--file",
            "README.md",
            "--id",
            "0",
            "--if-match",
            token(&baseline),
            "--record",
            r#"{"context":"Remote context"}"#,
        ],
    );
    let merged = success(
        root.path(),
        &[
            "merge",
            "--file",
            "README.md",
            "--id",
            "0",
            "--if-match",
            token(&remote),
            "--baseline-source",
            baseline["source"].as_str().unwrap(),
            "--record",
            r#"{"title":"Local title"}"#,
        ],
    );
    assert_eq!(merged["records"][0]["title"], "Local title");
    assert_eq!(merged["records"][0]["context"], "Remote context");
    let before = fs::read(root.path().join("README.md")).unwrap();
    let (status, error) = invoke(
        root.path(),
        &[
            "merge",
            "--file",
            "README.md",
            "--id",
            "0",
            "--if-match",
            token(&merged),
            "--baseline-source",
            baseline["source"].as_str().unwrap(),
            "--record",
            r#"{"title":"Competing title"}"#,
        ],
    );
    assert_eq!(status, 3, "{error}");
    assert_eq!(fs::read(root.path().join("README.md")).unwrap(), before);
}

#[test]
fn inputs_from_files_and_full_u64_identifiers_remain_lossless() {
    let root = TempDir::new().unwrap();
    fs::write(
        root.path().join("record.json"),
        json!({"title":"File input"}).to_string(),
    )
    .unwrap();
    success(
        root.path(),
        &[
            "create",
            "--file",
            "README.md",
            "--if-match",
            "missing",
            "--record-file",
            "record.json",
        ],
    );
    let source = fs::read_to_string(root.path().join("README.md"))
        .unwrap()
        .replace("### 0 ", "### 18446744073709551615 ");
    fs::write(root.path().join("README.md"), &source).unwrap();
    let shown = success(
        root.path(),
        &[
            "show",
            "--file",
            "README.md",
            "--id",
            "18446744073709551615",
        ],
    );
    assert_eq!(shown["record"]["id"], "18446744073709551615");
    fs::write(root.path().join("baseline.md"), source).unwrap();
    fs::write(
        root.path().join("record.json"),
        json!({"title":"Changed safely"}).to_string(),
    )
    .unwrap();
    success(
        root.path(),
        &[
            "merge",
            "--file",
            "README.md",
            "--id",
            "18446744073709551615",
            "--if-match",
            token(&shown),
            "--baseline-file",
            "baseline.md",
            "--record-file",
            "record.json",
        ],
    );
}

#[test]
fn help_is_json_and_discovery_ignores_hidden_generated_and_symlinked_files() {
    let root = TempDir::new().unwrap();
    let (status, help) = invoke(root.path(), &["--help"]);
    assert_eq!(status, 0);
    assert!(help["help"].as_str().unwrap().contains("--if-match"));
    create(root.path(), "visible.md", "Visible", "missing");
    for directory in [
        ".hidden",
        "target",
        "vendor",
        "node_modules",
        "dist",
        "build",
        "venv",
        "__pycache__",
    ] {
        fs::create_dir(root.path().join(directory)).unwrap();
        fs::write(
            root.path().join(directory).join("bad.md"),
            "<!-- vrdx start -->broken",
        )
        .unwrap();
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(root.path().join("visible.md"), root.path().join("alias.md"))
        .unwrap();
    assert_eq!(
        success(root.path(), &["list"])["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}
