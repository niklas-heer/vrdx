//! Keep copyable README examples coupled to the shipped parser.

use std::path::PathBuf;
use vrdx::document::Document;

#[test]
fn readme_markdown_example_is_a_discoverable_record() {
    let readme = include_str!("../README.md");
    let example = readme
        .split("```markdown\n")
        .nth(1)
        .unwrap()
        .split("```")
        .next()
        .unwrap();
    let document = Document::parse(PathBuf::from("README-example.md"), example.to_owned()).unwrap();
    assert!(document.has_markers);
    assert_eq!(document.records.len(), 1);
}

#[test]
fn repository_decision_history_remains_readable() {
    let document = Document::parse(
        PathBuf::from("README.md"),
        include_str!("../README.md").to_owned(),
    )
    .unwrap();
    assert!(document.records.iter().any(|record| record.id == 1));
    assert!(document.records.iter().any(|record| record.id == 2));
}
