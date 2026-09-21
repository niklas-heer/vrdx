//! Verify the public format example and reproducible toolchain declaration.

#[test]
fn readme_record_is_accepted_by_the_collection_parser() {
    let readme = include_str!("../README.md");
    let example = readme
        .split("```markdown\n")
        .nth(1)
        .unwrap()
        .split("\n```")
        .next()
        .unwrap();
    let root = tempfile::TempDir::new().unwrap();
    std::fs::write(root.path().join("example.md"), example).unwrap();
    let graph = vrdx::records::Graph::load(root.path()).unwrap();
    assert!(graph.findings.is_empty(), "{:?}", graph.findings);
    assert_eq!(graph.decisions.len(), 1);
    assert!(
        graph
            .decisions
            .values()
            .any(|record| record.body == example.split_once("\n+++\n").unwrap().1)
    );
}

#[test]
fn mise_and_cargo_pin_the_same_toolchain() {
    let mise: toml::Value = toml::from_str(include_str!("../mise.toml")).unwrap();
    let rust: toml::Value = toml::from_str(include_str!("../rust-toolchain.toml")).unwrap();
    let cargo: toml::Value = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    assert_eq!(
        mise["tools"]["rust"]["version"],
        rust["toolchain"]["channel"]
    );
    assert_eq!(
        cargo["package"]["rust-version"],
        rust["toolchain"]["channel"]
    );
}

#[test]
fn repository_skill_files_match_the_embedded_copies() {
    let binary =
        std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into());
    let output = std::process::Command::new(binary)
        .args(["init", "--dry-run", "--json"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    let response: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(output.status.success(), "{response}");
    // Only the embedded files are checked: CI containers receive a filtered source tree.
    let skill_paths: Vec<_> = response["data"]["paths"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|path| {
            path["path"]
                .as_str()
                .is_some_and(|path| path.starts_with(".agents/skills/vrdx/"))
        })
        .collect();
    assert_eq!(skill_paths.len(), 2, "{response}");
    assert!(
        skill_paths.iter().all(|path| path["status"] == "unchanged"),
        "run vrdx init in the repository root: {response}"
    );
}
