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
            .any(|record| record.body.contains("briefly stale data"))
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
