//! Public authoring contracts, recovery, formatting invariants and relocation.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Fixtures fail immediately on invalid setup or responses"
)]

use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};
use tempfile::TempDir;

fn binary() -> std::ffi::OsString {
    std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into())
}
fn run(root: &Path, args: &[&str], input: Option<&str>) -> Output {
    let mut child = Command::new(binary())
        .args(["--dir", "decisions", "--json"])
        .args(args)
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    if let Some(input) = input {
        stdin.write_all(input.as_bytes()).unwrap();
    }
    drop(stdin);
    child.wait_with_output().unwrap()
}
fn response(output: &Output) -> Value {
    assert!(output.stderr.is_empty(), "{output:?}");
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["schema_version"], 1);
    result
}
fn success(root: &Path, args: &[&str], input: Option<&str>) -> Value {
    let output = run(root, args, input);
    let result = response(&output);
    assert!(output.status.success(), "{result}");
    assert_eq!(result["ok"], true);
    result["data"].clone()
}
fn payload() -> Value {
    json!({"title":"Cache for one minute","decision":"Cache successful reads for 60 seconds.","why":"Repeated reads are expensive.","consequences":["Fewer requests.","Stale reads for a minute."],"tags":["performance"]})
}

#[test]
fn guide_example_drives_creation_query_and_format_without_handcrafted_metadata() {
    let root = TempDir::new().unwrap();
    let guide = success(root.path(), &["guide"], None);
    let example = guide["new_input"]["example"].to_string();
    let created = success(root.path(), &["new", "--from-json", "-"], Some(&example));
    let record = &created["decision"];
    assert_eq!(record["status"], "proposed");
    assert_eq!(record["title"], guide["new_input"]["example"]["title"]);
    assert!(record["body"].as_str().unwrap().contains("## Why\n"));
    let id = record["id"].as_str().unwrap();
    assert_eq!(id.len(), 26);
    assert_eq!(
        success(root.path(), &["show", id], None)["decision"],
        *record
    );
    assert_eq!(success(root.path(), &["validate"], None)["valid"], true);
    assert_eq!(
        success(root.path(), &["fmt", "--check"], None)["files"],
        json!([])
    );
    let source = fs::read_to_string(
        root.path()
            .join("decisions")
            .join(record["file"].as_str().unwrap()),
    )
    .unwrap();
    assert!(!source.contains("related_to = []"));
    assert_eq!(guide["new_input"]["schema"]["additionalProperties"], false);
    for name in ["new", "prompt", "fmt", "validate", "guide"] {
        assert!(
            guide["commands"]
                .as_array()
                .unwrap()
                .iter()
                .any(|entry| entry["name"] == name)
        );
        assert!(run(root.path(), &[name, "--help"], None).status.success());
    }
}

#[test]
fn malformed_json_and_conflicting_flags_never_create_a_collection() {
    let root = TempDir::new().unwrap();
    let mut unknown = payload();
    unknown["approval"] = json!("invented");
    let mut blank = payload();
    blank["why"] = json!(" \n ");
    let mut empty = payload();
    empty["consequences"] = json!([]);
    for input in [
        "{}".into(),
        "not json".into(),
        format!("{} {}", payload(), payload()),
        unknown.to_string(),
        blank.to_string(),
        empty.to_string(),
    ] {
        let output = run(root.path(), &["new", "--from-json", "-"], Some(&input));
        let result = response(&output);
        assert_eq!(output.status.code(), Some(2), "{result}");
        assert_eq!(result["error"]["code"], "invalid_input");
        assert!(result["error"]["hint"].as_str().unwrap().contains("guide"));
        assert!(!root.path().join("decisions").exists());
    }
    for args in [
        vec!["new", "Title", "--from-json", "-"],
        vec!["new", "--from-json", "-", "--tag", "x"],
        vec!["new", "Title", "--edit"],
    ] {
        let output = run(root.path(), &args, None);
        assert_eq!(
            output.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(!root.path().join("decisions").exists());
    }
}

#[test]
fn json_file_input_and_copyable_prompt_are_composable() {
    let root = TempDir::new().unwrap();
    let prompt = success(root.path(), &["prompt", "Cache for one minute"], None);
    let text = prompt["prompt"].as_str().unwrap();
    assert!(text.contains("Cache for one minute"));
    assert!(text.contains("My notes:"));
    assert!(text.contains("vrdx new --from-json decision.json --json"));
    assert!(!root.path().join("decisions").exists());
    fs::write(root.path().join("input.json"), payload().to_string()).unwrap();
    let record = success(root.path(), &["new", "--from-json", "input.json"], None);
    assert_eq!(record["decision"]["title"], "Cache for one minute");
    let human = Command::new(binary())
        .args(["prompt", "Use local files"])
        .current_dir(root.path())
        .output()
        .unwrap();
    assert!(human.status.success());
    assert!(
        String::from_utf8(human.stdout)
            .unwrap()
            .starts_with("Help me capture")
    );
}

#[test]
fn bounded_json_input_rejects_wrong_types_null_and_invalid_encoding() {
    let root = TempDir::new().unwrap();
    let path = root.path().join("input.json");
    let mut inputs = vec![vec![b' '; 1_048_577], vec![0xff, 0xfe]];
    for (field, value) in [
        ("date", Value::Null),
        ("status", Value::Null),
        ("tags", json!("cache")),
    ] {
        let mut input = payload();
        input[field] = value;
        inputs.push(input.to_string().into_bytes());
    }
    for input in inputs {
        fs::write(&path, input).unwrap();
        let output = run(root.path(), &["new", "--from-json", "input.json"], None);
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(response(&output)["error"]["code"], "invalid_input");
        assert!(!root.path().join("decisions").exists());
    }
}

#[test]
fn formatting_is_idempotent_and_preserves_body_comments_identity_and_permissions() {
    let root = TempDir::new().unwrap();
    let created = success(
        root.path(),
        &["new", "--from-json", "-"],
        Some(&payload().to_string()),
    );
    let record = &created["decision"];
    let path = root
        .path()
        .join("decisions")
        .join(record["file"].as_str().unwrap());
    let original = fs::read_to_string(&path).unwrap();
    let body = "\r\n## Custom 日本語\r\n\r\nHard break  \r\nnext line\r\n\r\n```text\r\n  tabs\tand spaces  \r\n```\r\n\r\n<!-- preserve me -->\r\n";
    for spaces in ["", " ", "   ", "\t"] {
        let source = format!(
            "+++\n# title comment\ntitle{spaces}={spaces}\"Cache for one minute\" # inline comment\n# identity comment\nid = \"{}\"\nstatus = \"proposed\"\ndate = \"2026-09-19\"\nschema_version = 1\ntags = [\n  # tag comment\n  \"performance\",\n]\n+++\n{body}",
            record["id"].as_str().unwrap()
        );
        fs::write(&path, &source).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();
        }
        let check = run(root.path(), &["fmt", "--check"], None);
        assert_eq!(check.status.code(), Some(1));
        assert_eq!(response(&check)["data"]["files"], json!([record["file"]]));
        assert_eq!(fs::read_to_string(&path).unwrap(), source);
        success(root.path(), &["fmt"], None);
        let formatted = fs::read_to_string(&path).unwrap();
        for comment in [
            "# title comment",
            "# inline comment",
            "# identity comment",
            "# tag comment",
        ] {
            assert!(formatted.contains(comment));
        }
        assert!(formatted.contains("title = \"Cache for one minute\""));
        assert!(formatted.ends_with(body));
        let shown = success(root.path(), &["show", record["id"].as_str().unwrap()], None);
        assert_eq!(shown["decision"]["body"], body);
        assert_eq!(shown["decision"]["tags"], json!(["performance"]));
        assert_eq!(
            success(root.path(), &["fmt", "--check"], None)["files"],
            json!([])
        );
        success(root.path(), &["fmt"], None);
        assert_eq!(fs::read_to_string(&path).unwrap(), formatted);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&path).unwrap().permissions().mode() & 0o777,
                0o640
            );
        }
    }
    // Invalid sibling prevents all writes, even when the first record needs formatting.
    fs::write(&path, original.replace("title =", "title=")).unwrap();
    fs::write(root.path().join("decisions/broken.md"), "broken").unwrap();
    let before = fs::read(&path).unwrap();
    assert!(!run(root.path(), &["fmt"], None).status.success());
    assert_eq!(fs::read(&path).unwrap(), before);
}

#[test]
fn validation_hints_help_humans_and_agents_repair_a_broken_reference() {
    let root = TempDir::new().unwrap();
    let created = success(
        root.path(),
        &["new", "--from-json", "-"],
        Some(&payload().to_string()),
    );
    let file = created["decision"]["file"].as_str().unwrap();
    let path = root.path().join("decisions").join(file);
    let source = fs::read_to_string(&path).unwrap();
    let broken = source.replacen(
        "+++\n",
        "+++\nrelated_to = [\"01ARZ3NDEKTSV4RRFFQ69G5FAV\"]\n",
        1,
    );
    fs::write(&path, &broken).unwrap();
    let result = response(&run(root.path(), &["validate"], None));
    let finding = &result["data"]["findings"][0];
    assert_eq!(finding["file"], file);
    assert_eq!(finding["code"], "missing_reference");
    assert!(
        finding["hint"]
            .as_str()
            .unwrap()
            .contains("existing full ID")
    );
    let human = Command::new(binary())
        .arg("validate")
        .current_dir(root.path())
        .output()
        .unwrap();
    let text = String::from_utf8(human.stdout).unwrap();
    assert!(text.contains(file));
    assert!(text.contains("Fix:"));
    assert_eq!(fs::read_to_string(&path).unwrap(), broken);
    fs::write(&path, source).unwrap();
    success(root.path(), &["validate"], None);
}

#[cfg(unix)]
#[test]
fn editor_uses_quoted_arguments_and_preserves_failed_drafts() {
    use std::os::unix::fs::PermissionsExt;
    let root = TempDir::new().unwrap();
    let editor = root.path().join("my editor");
    fs::write(&editor, "#!/bin/sh\ntest \"$1\" = '--wait' || exit 3\nshift\n/usr/bin/sed 's/One sentence: what are we choosing?/Use a local cache./' \"$1\" > \"$1.edited\"\n/bin/mv \"$1.edited\" \"$1\"\n").unwrap();
    fs::set_permissions(&editor, fs::Permissions::from_mode(0o700)).unwrap();
    let output = Command::new(binary())
        .args(["new", "Local cache", "--edit"])
        .env("VISUAL", format!("'{}' --wait", editor.display()))
        .env("EDITOR", "/no/such/editor")
        .current_dir(root.path())
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let all = success(root.path(), &["rebuild"], None);
    let records = all["graph"]["decisions"].as_object().unwrap();
    assert_eq!(records.len(), 1);
    assert!(
        records.values().next().unwrap()["body"]
            .as_str()
            .unwrap()
            .contains("Use a local cache.")
    );
    for script in [
        "#!/bin/sh\nexit 7\n",
        "#!/bin/sh\nprintf 'bad metadata' > \"$1\"\n",
    ] {
        fs::write(&editor, script).unwrap();
        let output = Command::new(binary())
            .args(["new", "Failed draft", "--edit"])
            .env_remove("VISUAL")
            .env("EDITOR", format!("'{}'", editor.display()))
            .current_dir(root.path())
            .output()
            .unwrap();
        assert!(!output.status.success());
        let error = String::from_utf8(output.stderr).unwrap();
        let retained = error
            .split("Draft retained at ")
            .nth(1)
            .unwrap()
            .split(". Repair it")
            .next()
            .unwrap();
        assert!(Path::new(retained).is_file());
        fs::remove_file(retained).unwrap();
        assert_eq!(
            success(root.path(), &["rebuild"], None)["graph"]["decisions"]
                .as_object()
                .unwrap()
                .len(),
            1
        );
    }
}

#[test]
fn one_relocated_executable_bootstraps_without_runtime_tools_or_assets() {
    let root = TempDir::new().unwrap();
    let executable = root.path().join("vrdx-alone");
    fs::copy(binary(), &executable).unwrap();
    fs::write(root.path().join("input.json"), payload().to_string()).unwrap();
    for args in [
        vec!["guide"],
        vec!["prompt", "Local cache"],
        vec!["new", "--from-json", "input.json"],
        vec!["validate"],
        vec!["fmt", "--check"],
    ] {
        let output = Command::new(&executable)
            .arg("--json")
            .args(&args)
            .env("PATH", root.path().join("no-tools"))
            .env_remove("VISUAL")
            .env_remove("EDITOR")
            .current_dir(root.path())
            .output()
            .unwrap();
        assert!(output.status.success(), "{args:?}: {output:?}");
        assert_eq!(response(&output)["ok"], true);
    }
}
