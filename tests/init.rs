//! `init` installs the bundled agent skill idempotently and never touches other content.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Fixtures fail immediately on invalid setup or response shapes"
)]

use serde_json::Value;
use std::{fs, path::Path, process::Command};
use tempfile::TempDir;

const SKILL: &str = ".agents/skills/vrdx/SKILL.md";
const ONBOARDING: &str = ".agents/skills/vrdx/references/onboarding.md";
const LINK: &str = ".claude/skills/vrdx";

fn run(root: &Path, args: &[&str]) -> (i32, Value) {
    let binary =
        std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into());
    let output = Command::new(binary)
        .arg("--json")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(output.stderr.is_empty(), "{output:?}");
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(response["schema_version"], 1);
    (output.status.code().unwrap(), response)
}

fn statuses(data: &Value) -> Vec<(String, String)> {
    data["paths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|path| {
            (
                path["path"].as_str().unwrap().to_owned(),
                path["status"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

fn init(root: &Path, args: &[&str]) -> Value {
    let (code, response) = run(root, &[&["init"], args].concat());
    assert_eq!(code, 0, "{response}");
    assert_eq!(response["ok"], true);
    response["data"].clone()
}

fn agents_block(root: &Path) -> String {
    let agents = fs::read_to_string(root.join("AGENTS.md")).unwrap();
    let start = agents.find("<!-- vrdx:start -->").unwrap();
    let end = agents.find("<!-- vrdx:end -->").unwrap();
    assert!(start < end);
    assert_eq!(agents.matches("<!-- vrdx:start -->").count(), 1);
    agents.get(start..end).unwrap().to_owned()
}

#[test]
fn fresh_project_gets_skill_symlink_and_agents_file_then_stays_unchanged() {
    let root = TempDir::new().unwrap();
    let data = init(root.path(), &[]);
    assert_eq!(data["dry_run"], false);
    assert_eq!(data["collection"], "decisions");
    assert!(
        statuses(&data)
            .iter()
            .all(|(_, status)| status == "created"),
        "{data}"
    );
    assert_eq!(
        statuses(&data)
            .iter()
            .map(|(path, _)| path.as_str())
            .collect::<Vec<_>>(),
        [SKILL, ONBOARDING, LINK, "AGENTS.md"]
    );
    let skill = fs::read_to_string(root.path().join(SKILL)).unwrap();
    assert_eq!(skill, include_str!("../.agents/skills/vrdx/SKILL.md"));
    assert!(root.path().join(ONBOARDING).is_file());
    let link = root.path().join(LINK);
    assert!(
        fs::symlink_metadata(&link)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::read_link(&link).unwrap(),
        Path::new("../../.agents/skills/vrdx")
    );
    assert_eq!(
        fs::read_to_string(link.join("SKILL.md")).unwrap(),
        skill,
        "Claude Code reads the skill through the link"
    );
    let block = agents_block(root.path());
    assert!(block.contains("`decisions/`"));
    assert!(!block.contains("--dir"));
    assert!(block.contains(SKILL));
    assert_eq!(data["onboarding"]["candidates"], serde_json::json!([]));
    assert_eq!(data["onboarding"]["collection_needs_import"], false);
    assert!(!root.path().join("decisions").exists());

    let again = init(root.path(), &[]);
    assert!(
        statuses(&again)
            .iter()
            .all(|(_, status)| status == "unchanged"),
        "{again}"
    );
}

#[test]
fn existing_agents_content_is_preserved_and_the_block_is_replaced_in_place() {
    let root = TempDir::new().unwrap();
    let original = "# Project\n\nKeep this line.\n\n## Testing\nRun the tests.";
    fs::write(root.path().join("AGENTS.md"), original).unwrap();
    let data = init(root.path(), &[]);
    assert!(
        statuses(&data)
            .iter()
            .any(|(path, status)| path == "AGENTS.md" && status == "updated")
    );
    let agents = fs::read_to_string(root.path().join("AGENTS.md")).unwrap();
    assert!(agents.starts_with(original));
    assert!(agents.contains("\n\n<!-- vrdx:start -->"));
    assert!(agents.ends_with("<!-- vrdx:end -->\n"));

    let edited = agents.replace(
        "<!-- vrdx:end -->\n",
        "<!-- vrdx:end -->\n\n## After\nTail.\n",
    );
    fs::write(root.path().join("AGENTS.md"), &edited).unwrap();
    let stale = edited.replace("Consequential", "Stale");
    fs::write(root.path().join("AGENTS.md"), &stale).unwrap();
    let data = init(root.path(), &[]);
    assert!(
        statuses(&data)
            .iter()
            .any(|(path, status)| path == "AGENTS.md" && status == "updated")
    );
    assert_eq!(
        fs::read_to_string(root.path().join("AGENTS.md")).unwrap(),
        edited
    );
}

#[test]
fn custom_collection_directory_is_named_in_the_block() {
    let root = TempDir::new().unwrap();
    let data = init(root.path(), &["--dir", "docs/decisions"]);
    assert_eq!(data["collection"], "docs/decisions");
    let block = agents_block(root.path());
    assert!(block.contains("`docs/decisions/`"));
    assert!(block.contains("--dir docs/decisions"));
}

#[test]
fn dry_run_reports_without_writing() {
    let root = TempDir::new().unwrap();
    let data = init(root.path(), &["--dry-run"]);
    assert_eq!(data["dry_run"], true);
    assert_eq!(statuses(&data).len(), 4);
    assert!(
        statuses(&data)
            .iter()
            .all(|(_, status)| status == "created")
    );
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[test]
fn differing_skill_content_is_updated_and_a_copied_claude_skill_is_refreshed() {
    let root = TempDir::new().unwrap();
    fs::create_dir_all(root.path().join(".agents/skills/vrdx")).unwrap();
    fs::write(root.path().join(SKILL), "old").unwrap();
    fs::create_dir_all(root.path().join(LINK)).unwrap();
    let data = init(root.path(), &[]);
    let statuses = statuses(&data);
    assert!(statuses.contains(&(SKILL.to_owned(), "updated".to_owned())));
    assert!(statuses.contains(&(format!("{LINK}/SKILL.md"), "created".to_owned())));
    assert!(!statuses.iter().any(|(path, _)| path == LINK));
    assert!(
        root.path()
            .join(LINK)
            .join("references/onboarding.md")
            .is_file()
    );
    assert_eq!(
        fs::read_to_string(root.path().join(SKILL)).unwrap(),
        fs::read_to_string(root.path().join(LINK).join("SKILL.md")).unwrap()
    );
}

#[test]
fn conflicts_abort_before_any_write() {
    let root = TempDir::new().unwrap();
    fs::create_dir_all(root.path().join(".claude/skills")).unwrap();
    fs::write(root.path().join(LINK), "not a directory").unwrap();
    let (code, response) = run(root.path(), &["init"]);
    assert_eq!(code, 3, "{response}");
    assert_eq!(response["ok"], false);
    assert_eq!(response["error"]["code"], "conflict");
    assert!(
        response["error"]["message"]
            .as_str()
            .unwrap()
            .contains(LINK)
    );
    assert!(!root.path().join(".agents").exists());
    assert!(!root.path().join("AGENTS.md").exists());

    fs::remove_file(root.path().join(LINK)).unwrap();
    fs::write(root.path().join("AGENTS.md"), "<!-- vrdx:end -->\n").unwrap();
    let (code, response) = run(root.path(), &["init"]);
    assert_eq!(code, 3, "{response}");
    assert!(!root.path().join(".agents").exists());
}

#[test]
fn onboarding_hints_report_known_folders_and_foreign_markdown() {
    let root = TempDir::new().unwrap();
    fs::create_dir_all(root.path().join("docs/adr")).unwrap();
    fs::create_dir_all(root.path().join("decisions")).unwrap();
    fs::write(
        root.path().join("decisions/0001-use-postgres.md"),
        "# 1. Use Postgres\n\nAccepted.\n",
    )
    .unwrap();
    let data = init(root.path(), &["--dry-run"]);
    assert_eq!(
        data["onboarding"]["candidates"],
        serde_json::json!(["docs/adr"])
    );
    assert_eq!(data["onboarding"]["collection_needs_import"], true);

    let data = init(root.path(), &["--dry-run", "--dir", "docs/adr"]);
    assert_eq!(data["onboarding"]["candidates"], serde_json::json!([]));
    assert_eq!(data["onboarding"]["collection_needs_import"], false);
}

#[test]
fn bundled_skill_follows_the_agent_skills_format() {
    let skill = include_str!("../.agents/skills/vrdx/SKILL.md");
    let frontmatter = skill
        .strip_prefix("---\n")
        .unwrap()
        .split("\n---\n")
        .next()
        .unwrap();
    let field = |key: &str| {
        frontmatter
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{key}: ")))
            .unwrap()
            .trim()
            .to_owned()
    };
    assert_eq!(field("name"), "vrdx", "name must match the directory");
    let description = field("description");
    assert!(!description.is_empty() && description.chars().count() <= 1024);
    assert!(
        description.contains("Not for"),
        "trigger needs an exclusion"
    );
    assert!(skill.lines().count() < 80, "the skill stays concise");
    assert!(skill.contains("references/onboarding.md"));
}
