// Tests return setup I/O errors with `?`; assertions remain the test oracle.
#![allow(clippy::panic_in_result_fn)]

use super::*;

fn complete_record(id: u64) -> Record {
    Record {
        id,
        title: "Use SQLite".into(),
        status: "📝 Draft".into(),
        decision: "First paragraph.\n\nSecond paragraph.".into(),
        context: "Local data".into(),
        consequences: "One file".into(),
    }
}

fn wrap(body: &str) -> String {
    format!("# Notes\n{START}\n{body}\n{END}\nTrailing text\n")
}

#[test]
fn premise_shape_preserves_max_identifier_unicode_and_empty_fields() {
    let mut record = Record::new(u64::MAX);
    record.title = "Décision 日本語 🦀".into();
    assert_eq!(Record::refielded(&record.field()), Some(record.clone()));
    assert_eq!(found_ref(&[record.clone()], u64::MAX), Some(&record));
}

#[test]
fn premise_shape_rejects_duplicate_unknown_missing_or_wrong_typed_fields() {
    let Field::Table(rows) = complete_record(0).field() else {
        return;
    };
    let mut duplicate = rows.clone();
    if let Some((key, _)) = duplicate.first_mut() {
        *key = "title".into();
    }
    let mut unknown = rows.clone();
    if let Some((key, _)) = unknown.first_mut() {
        *key = "unknown".into();
    }
    let missing: Vec<_> = rows
        .iter()
        .filter(|(name, _)| name != "status")
        .cloned()
        .collect();
    let wrong_type: Vec<_> = rows
        .iter()
        .map(|(name, field)| {
            (
                name.clone(),
                if name == "id" {
                    Field::Whole(0)
                } else {
                    field.clone()
                },
            )
        })
        .collect();
    let overflow: Vec<_> = rows
        .iter()
        .map(|(name, field)| {
            (
                name.clone(),
                if name == "id" {
                    Field::Text("18446744073709551616".into())
                } else {
                    field.clone()
                },
            )
        })
        .collect();
    for shape in [
        Field::Nothing,
        Field::Table(duplicate),
        Field::Table(unknown),
        Field::Table(missing),
        Field::Table(wrong_type),
        Field::Table(overflow),
    ] {
        assert!(Record::refielded(&shape).is_none());
    }
}

#[test]
fn title_only_roundtrip() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decisions.md");
    let mut document = Document::new(path.clone());
    let mut record = Record::new(0);
    record.title = "Choose later".into();
    document.save_record(&record)?;
    assert_eq!(Document::load(path)?.records, vec![record]);
    Ok(())
}

#[test]
fn noop_preserves_bom_crlf_and_timestamp() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decisions.md");
    let record = complete_record(1);
    let source = format!("\u{feff}{}", wrap(&record.render("\n"))).replace('\n', "\r\n");
    fs::write(&path, &source)?;
    let timestamp = fs::metadata(&path)?.modified()?;
    let mut document = Document::load(&path)?;
    document.save_record(&record)?;
    assert_eq!(fs::read_to_string(&path)?, source);
    assert_eq!(fs::metadata(&path)?.modified()?, timestamp);
    Ok(())
}

#[test]
fn leading_narrative_indentation_survives_noop_and_another_field_edit() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let mut record = complete_record(0);
    record.decision = "    preserve this indentation\n\nNext paragraph.".into();
    let source = wrap(&record.render("\r\n"));
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    document.save_record(&record)?;
    assert_eq!(fs::read_to_string(&path)?, source);
    record.title = "Changed title".into();
    document.save_record(&record)?;
    assert_eq!(Document::load(path)?.records, vec![record]);
    Ok(())
}

#[cfg(unix)]
#[test]
fn new_file_refuses_live_and_dangling_symlinks_without_touching_target() -> Result<(), Error> {
    use std::os::unix::fs::symlink;
    let repository = tempfile::tempdir()?;
    let external = tempfile::tempdir()?;
    for existing in [false, true] {
        let target = external.path().join(if existing {
            "existing.md"
        } else {
            "missing.md"
        });
        let link = repository
            .path()
            .join(if existing { "live.md" } else { "dangling.md" });
        if existing {
            fs::write(&target, "External original")?;
        }
        symlink(&target, &link)?;
        let mut document = Document::new(link.clone());
        assert!(matches!(
            document.save_record(&complete_record(0)),
            Err(Error::Conflict(_))
        ));
        assert!(fs::symlink_metadata(link)?.is_symlink());
        if existing {
            assert_eq!(fs::read_to_string(target)?, "External original");
        } else {
            assert!(!target.exists());
        }
    }
    Ok(())
}

#[test]
fn edit_preserves_other_records_preamble_and_crlf() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decisions.md");
    let mut record = complete_record(2);
    let other = complete_record(1)
        .render("\r\n")
        .replace("### 1 ", "### 1. ");
    let original_record = record.render("\r\n");
    let source = format!(
        "\u{feff}# Notes\r\n{START}\r\nPreamble\r\n\r\n{original_record}\r\n\r\n\r\n{other}\r\n{END}\r\nFooter\r\n"
    );
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    record.context = "Changed context".into();
    document.save_record(&record)?;
    assert_eq!(
        fs::read_to_string(path)?,
        source.replace(&original_record, &record.render("\r\n"))
    );
    Ok(())
}

#[test]
fn fenced_and_inline_examples_are_not_markers_or_headings() -> Result<(), Error> {
    let examples = format!("```html\n{START}\n{END}\n```\n``{START}`` and `{END}`\n");
    assert!(!Document::parse(PathBuf::new(), examples.clone())?.has_markers);
    let mut record = complete_record(0);
    record.decision = format!(
        "An example:\n\n~~~~markdown\n### 42 Not a decision\n* **Status**: example\n{START}\n{END}\n~~~~\n\nMore prose."
    );
    let source = format!("{examples}{}", wrap(&record.render("\n")));
    assert_eq!(
        Document::parse(PathBuf::new(), source)?.records,
        vec![record]
    );
    Ok(())
}

#[test]
fn malformed_markers_ids_and_fields_are_rejected() {
    let body = complete_record(1).render("\n");
    for source in [
        START.to_owned(),
        format!("{END}\n{START}"),
        format!("{}\n{}", wrap(&body), wrap(&body)),
        wrap(&format!("{body}\n\n{body}")),
        wrap(&body.replace("* **Context**: Local data", "")),
        wrap(&body.replace("* **Status**:", "Unrecognized introduction\n* **Status**:")),
        wrap(&body.replace(
            "* **Context**: Local data",
            "* **Context**: one\n* **Context**: two",
        )),
        wrap(&body.replace("### 1 ", "### 18446744073709551616 ")),
    ] {
        assert!(matches!(
            Document::parse(PathBuf::new(), source),
            Err(Error::Invalid(_))
        ));
    }
}

#[test]
fn external_edits_are_not_overwritten_and_snapshot_retained() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decisions.md");
    let mut record = complete_record(1);
    let source = wrap(&record.render("\n"));
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    let external = format!("{source}\nExternal change\n");
    fs::write(&path, &external)?;
    record.title = "My draft".into();
    assert!(matches!(
        document.save_record(&record),
        Err(Error::Conflict(_))
    ));
    assert_eq!(fs::read_to_string(path)?, external);
    assert_eq!(document.source, source);
    assert_eq!(record.title, "My draft");
    Ok(())
}

#[test]
fn final_check_catches_change_and_removes_tempfile() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decisions.md");
    let mut record = complete_record(1);
    fs::write(&path, wrap(&record.render("\n")))?;
    let mut document = Document::load(&path)?;
    record.title = "Changed".into();
    let result = document.save_with_hook(&record, || {
        fs::write(&path, "Concurrent update")?;
        Ok(())
    });
    assert!(matches!(result, Err(Error::Conflict(_))));
    assert_eq!(fs::read_to_string(&path)?, "Concurrent update");
    assert_eq!(fs::read_dir(dir.path())?.count(), 1);
    Ok(())
}

#[test]
fn failure_before_replacement_keeps_original_and_cleans_tempfile() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decisions.md");
    let mut record = complete_record(1);
    let source = wrap(&record.render("\n"));
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    record.title = "Changed".into();
    let result = document.save_with_hook(&record, || {
        Err(Error::Io(std::io::Error::other("Injected failure")))
    });
    assert!(matches!(result, Err(Error::Io(_))));
    assert_eq!(fs::read_to_string(&path)?, source);
    assert_eq!(document.source, source);
    assert_eq!(fs::read_dir(dir.path())?.count(), 1);
    Ok(())
}

#[test]
fn new_file_never_clobbers_existing_target() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decisions.md");
    let mut document = Document::new(path.clone());
    fs::write(&path, "Existing document")?;
    assert!(matches!(
        document.save_record(&complete_record(1)),
        Err(Error::Conflict(_))
    ));
    assert_eq!(fs::read_to_string(path)?, "Existing document");
    Ok(())
}

#[test]
fn markerless_append_preserves_every_existing_byte() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = "\u{feff}# Notes\r\nMy original prose";
    fs::write(&path, source)?;
    let mut document = Document::load(&path)?;
    let record = complete_record(0);
    document.save_record(&record)?;
    assert!(fs::read_to_string(&path)?.starts_with(source));
    assert_eq!(Document::load(path)?.records, vec![record]);
    Ok(())
}

#[test]
fn initialization_refuses_unterminated_code_fence_without_changing_file() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = "# Example\n\n```markdown\nUnclosed code example\n";
    fs::write(&path, source)?;
    let mut document = Document::load(&path)?;
    assert!(matches!(
        document.save_record(&complete_record(0)),
        Err(Error::Invalid(_))
    ));
    assert_eq!(fs::read_to_string(path)?, source);
    Ok(())
}

#[test]
fn empty_marker_block_accepts_first_record() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    fs::write(&path, wrap("\n"))?;
    let mut document = Document::load(&path)?;
    let record = complete_record(0);
    document.save_record(&record)?;
    assert_eq!(Document::load(path)?.records, vec![record]);
    Ok(())
}

#[test]
fn ambiguous_edit_and_exhausted_identifier_are_rejected() -> Result<(), Error> {
    let record = complete_record(u64::MAX);
    let document = Document::parse(PathBuf::new(), wrap(&record.render("\n")))?;
    assert!(matches!(document.next_id(), Err(Error::IdOverflow)));
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let mut document = Document::new(path.clone());
    let mut ambiguous = complete_record(0);
    ambiguous.decision = "A paragraph\n* **Context**: injected".into();
    assert!(matches!(
        document.save_record(&ambiguous),
        Err(Error::Invalid(_))
    ));
    assert!(!path.exists());
    Ok(())
}

#[cfg(unix)]
#[test]
fn unix_permissions_and_symlink_are_preserved() -> Result<(), Error> {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let link = dir.path().join("link.md");
    fs::write(&path, wrap(&complete_record(1).render("\n")))?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o640))?;
    symlink(&path, &link)?;
    let mut document = Document::load(&link)?;
    let mut record = complete_record(1);
    record.title = "Changed title".into();
    document.save_record(&record)?;
    assert!(fs::symlink_metadata(&link)?.is_symlink());
    assert_eq!(fs::metadata(path)?.permissions().mode() & 0o777, 0o640);
    Ok(())
}

fn three_record_source() -> String {
    let first = complete_record(9)
        .render("\r\n")
        .replace("### 9 ", "### 9. ");
    let middle = complete_record(4).render("\r\n");
    let last = complete_record(17).render("\r\n");
    format!(
        "\u{feff}# Notes\r\n{START}\r\nPreamble\r\n\r\n{first}\r\n\r\n\r\n{middle}\r\n\r\n{last}\r\n{END}\r\nFooter\r\n"
    )
}

#[test]
fn deleting_first_middle_last_and_all_preserves_every_other_byte() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = three_record_source();
    for id in [9, 4, 17] {
        fs::write(&path, &source)?;
        let mut document = Document::load(&path)?;
        let index = document.record_index(id)?;
        let expected = with_high_water(&splice(&source, document.spans[index].clone(), "")?, 17)?;
        document.delete_record(id)?;
        assert_eq!(fs::read_to_string(&path)?, expected);
        assert_eq!(Document::load(&path)?.records.len(), 2);
        assert!(found_ref(&document.records, id).is_none());
    }
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    for id in [9, 4, 17] {
        document.delete_record(id)?;
    }
    let empty = fs::read_to_string(&path)?;
    assert!(empty.contains("Preamble\r\n"));
    assert!(empty.ends_with("Footer\r\n"));
    assert!(empty.starts_with('\u{feff}'));
    assert_eq!(Document::load(&path)?.records, Vec::<Record>::new());
    assert_eq!(document.next_id()?, 18);
    document.save_record(&complete_record(18))?;
    assert_eq!(Document::load(path)?.records, vec![complete_record(18)]);
    Ok(())
}

#[test]
fn moving_records_preserves_raw_contents_separators_and_surroundings() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = three_record_source();
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    let first = slice(&source, document.spans[0].clone())?;
    let middle = slice(&source, document.spans[1].clone())?;
    let last = slice(&source, document.spans[2].clone())?;
    document.move_record(9, 2)?;
    assert_eq!(
        document
            .records
            .iter()
            .map(|record| record.id)
            .collect::<Vec<_>>(),
        [4, 17, 9]
    );
    let expected = format!(
        "\u{feff}# Notes\r\n{START}\r\nPreamble\r\n\r\n{middle}\r\n\r\n\r\n{last}\r\n\r\n{first}\r\n{END}\r\nFooter\r\n"
    );
    assert_eq!(fs::read_to_string(&path)?, expected);
    document.move_record(9, 0)?;
    assert_eq!(fs::read_to_string(path)?, source);
    Ok(())
}

#[test]
fn move_noop_and_invalid_targets_never_write() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = three_record_source();
    fs::write(&path, &source)?;
    let timestamp = fs::metadata(&path)?.modified()?;
    let mut document = Document::load(&path)?;
    document.move_record(4, 1)?;
    assert!(matches!(document.move_record(4, 3), Err(Error::Invalid(_))));
    assert!(matches!(
        document.move_record(99, 0),
        Err(Error::Invalid(_))
    ));
    assert!(matches!(document.delete_record(99), Err(Error::Invalid(_))));
    assert_eq!(fs::read_to_string(&path)?, source);
    assert_eq!(fs::metadata(path)?.modified()?, timestamp);
    Ok(())
}

#[test]
fn stale_delete_and_move_preserve_external_changes_and_snapshot() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = three_record_source();
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    let external = format!("{source}External addition\r\n");
    fs::write(&path, &external)?;
    assert!(matches!(document.delete_record(4), Err(Error::Conflict(_))));
    assert!(matches!(
        document.move_record(4, 0),
        Err(Error::Conflict(_))
    ));
    assert_eq!(fs::read_to_string(path)?, external);
    assert_eq!(document.source, source);
    Ok(())
}

#[test]
fn merge_retains_independent_fields_records_and_latest_document_text() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = three_record_source();
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    let mut local = complete_record(4);
    local.title = "My new title".into();
    local.decision = "My new decision\n\nAnother paragraph".into();
    let mut external = Document::load(&path)?;
    let mut disk = complete_record(4);
    disk.status = "Accepted".into();
    disk.context = "External context".into();
    external.save_record(&disk)?;
    external.delete_record(17)?;
    let source = fs::read_to_string(&path)?.replace("Preamble", "External preamble");
    fs::write(&path, &source)?;
    document.merge_record(&local)?;
    let mut expected_record = local.clone();
    expected_record.status.clone_from(&disk.status);
    expected_record.context.clone_from(&disk.context);
    let expected_source = source.replace(&disk.render("\r\n"), &expected_record.render("\r\n"));
    assert_eq!(fs::read_to_string(&path)?, expected_source);
    assert_eq!(found_ref(&document.records, 4), Some(&expected_record));
    assert!(found_ref(&document.records, 17).is_none());
    assert_eq!(local.title, "My new title");
    assert_eq!(local.status, "📝 Draft");
    document.save_record(&expected_record)?;
    Ok(())
}

#[test]
fn merge_rejects_divergent_changes_for_every_field_without_mutation() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let base = complete_record(4);
    for field in 0..5 {
        fs::write(&path, wrap(&base.render("\n")))?;
        let mut document = Document::load(&path)?;
        let before = document.source.clone();
        let mut local = base.clone();
        let mut disk = base.clone();
        for (record, value) in [(&mut local, "Local change"), (&mut disk, "External change")] {
            let target = match field {
                0 => &mut record.title,
                1 => &mut record.status,
                2 => &mut record.decision,
                3 => &mut record.context,
                _ => &mut record.consequences,
            };
            *target = value.into();
        }
        let external_source = wrap(&disk.render("\n"));
        fs::write(&path, &external_source)?;
        assert!(matches!(
            document.merge_record(&local),
            Err(Error::Conflict(_))
        ));
        assert_eq!(document.source, before);
        assert_eq!(document.records, vec![base.clone()]);
        assert_eq!(fs::read_to_string(&path)?, external_source);
    }
    Ok(())
}

#[test]
fn merge_identical_changes_refreshes_snapshot_without_rewriting_disk() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let mut local = complete_record(4);
    fs::write(&path, wrap(&local.render("\n")))?;
    let mut document = Document::load(&path)?;
    local.title = "Shared change".into();
    let disk = wrap(&local.render("\n")).replace("### 4 ", "### 4. ");
    fs::write(&path, &disk)?;
    let timestamp = fs::metadata(&path)?.modified()?;
    document.merge_record(&local)?;
    assert_eq!(document.source, disk);
    assert_eq!(fs::read_to_string(&path)?, disk);
    assert_eq!(fs::metadata(&path)?.modified()?, timestamp);
    Ok(())
}

#[test]
fn merge_cannot_resurrect_deleted_target_or_overwrite_concurrent_new_file() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let local = complete_record(4);
    fs::write(&path, wrap(&local.render("\n")))?;
    let mut document = Document::load(&path)?;
    let before = document.source.clone();
    fs::write(&path, wrap(""))?;
    assert!(matches!(
        document.merge_record(&local),
        Err(Error::Conflict(_))
    ));
    assert_eq!(document.source, before);
    assert_eq!(fs::read_to_string(&path)?, wrap(""));
    fs::remove_file(&path)?;
    assert!(matches!(
        document.merge_record(&local),
        Err(Error::Conflict(_))
    ));
    assert!(!path.exists());
    let mut new_document = Document::new(path.clone());
    fs::write(&path, "Someone else's document")?;
    assert!(matches!(
        new_document.merge_record(&local),
        Err(Error::Conflict(_))
    ));
    assert_eq!(fs::read_to_string(path)?, "Someone else's document");
    Ok(())
}

#[test]
fn malformed_merge_target_preserves_disk_and_local_snapshot() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let local = complete_record(4);
    fs::write(&path, wrap(&local.render("\n")))?;
    let mut document = Document::load(&path)?;
    let before = document.source.clone();
    for invalid in [
        wrap("### 4"),
        wrap("### 4 Missing fields"),
        START.to_owned(),
    ] {
        fs::write(&path, &invalid)?;
        assert!(matches!(
            document.merge_record(&local),
            Err(Error::Invalid(_))
        ));
        assert_eq!(document.source, before);
        assert_eq!(fs::read_to_string(&path)?, invalid);
    }
    Ok(())
}

#[test]
fn orphan_decision_fields_and_missing_heading_parts_are_rejected() {
    let record = complete_record(1).render("\n");
    for invalid in [
        record.replace("### 1 Use SQLite", "### Use SQLite"),
        record.replace("### 1 Use SQLite", "## 1 Use SQLite"),
        record.replace("### 1 Use SQLite", "### 1"),
        record.replace("### 1 Use SQLite", "### 18446744073709551616 Title"),
    ] {
        assert!(matches!(
            Document::parse(PathBuf::new(), wrap(&invalid)),
            Err(Error::Invalid(_))
        ));
    }
}

#[test]
fn deleted_identifiers_remain_reserved_after_reopening_and_creation() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    fs::write(&path, three_record_source())?;
    let mut document = Document::load(&path)?;
    document.delete_record(17)?;
    let mut reopened = Document::load(&path)?;
    assert_eq!(reopened.next_id()?, 18);
    assert!(matches!(
        reopened.save_record(&complete_record(17)),
        Err(Error::Invalid(_))
    ));
    reopened.save_record(&complete_record(18))?;
    reopened.delete_record(18)?;
    assert_eq!(Document::load(&path)?.next_id()?, 19);
    reopened.delete_record(9)?;
    reopened.delete_record(4)?;
    assert_eq!(Document::load(&path)?.next_id()?, 19);
    let source = fs::read_to_string(&path)?;
    assert_eq!(source.matches(HIGH_WATER).count(), 1);
    assert!(source.contains("<!-- vrdx high-water: 18 -->\r\n<!-- vrdx start -->"));
    Ok(())
}

#[test]
fn deleted_max_identifier_stays_exhausted_and_cannot_be_reused() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    fs::write(&path, wrap(&complete_record(u64::MAX).render("\n")))?;
    let mut document = Document::load(&path)?;
    document.delete_record(u64::MAX)?;
    let mut reopened = Document::load(&path)?;
    assert!(matches!(reopened.next_id(), Err(Error::IdOverflow)));
    assert!(matches!(
        reopened.save_record(&complete_record(u64::MAX)),
        Err(Error::Invalid(_))
    ));
    assert!(matches!(
        reopened.save_record(&complete_record(0)),
        Err(Error::Invalid(_))
    ));
    Ok(())
}

#[test]
fn malformed_duplicate_and_misplaced_reserved_metadata_are_rejected() -> Result<(), Error> {
    let valid = wrap(&complete_record(1).render("\n"));
    for malformed in [
        "<!-- vrdx high-water: nope -->",
        "<!-- vrdx high-water: 18446744073709551616 -->",
        "<!-- vrdx high-water: -1 -->",
        "<!-- vrdx high-water:  -->",
        "<!-- vrdx high-water -->",
        "<!-- vrdx high-water: 1",
        "<!-- vrdx high-water: 1 -->\n<!-- vrdx high-water: 2 -->",
    ] {
        assert!(matches!(
            Document::parse(PathBuf::new(), format!("{malformed}\n{valid}")),
            Err(Error::Invalid(_))
        ));
    }
    assert!(matches!(
        Document::parse(
            PathBuf::new(),
            format!("{valid}<!-- vrdx high-water: 1 -->")
        ),
        Err(Error::Invalid(_))
    ));
    let examples = format!("```html\n<!-- vrdx high-water: nope -->\n```\n{valid}");
    assert_eq!(Document::parse(PathBuf::new(), examples)?.next_id()?, 2);
    Ok(())
}

#[test]
fn merge_binds_verified_latest_snapshot_until_publication() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let base = complete_record(1);
    fs::write(&path, wrap(&base.render("\n")))?;
    let mut document = Document::load(&path)?;
    let original = document.source().to_owned();
    let verified_latest = Document::load(&path)?;
    let external = format!("{original}Post-verification update\n");
    fs::write(&path, &external)?;
    let mut local = base;
    local.title = "Local title".into();
    assert!(matches!(
        document.merge_record_against(&local, verified_latest),
        Err(Error::Conflict(_))
    ));
    assert_eq!(document.source(), original);
    assert_eq!(fs::read_to_string(&path)?, external);
    let current = Document::load(&path)?;
    document.merge_record_against(&local, current)?;
    assert_eq!(document.source(), fs::read_to_string(&path)?);
    assert!(document.source().ends_with("Post-verification update\n"));
    Ok(())
}

#[test]
fn merge_rejects_snapshot_for_a_different_file() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let other_path = dir.path().join("other.md");
    let source = wrap(&complete_record(1).render("\n"));
    fs::write(&path, &source)?;
    fs::write(&other_path, &source)?;
    let mut document = Document::load(&path)?;
    let other = Document::load(&other_path)?;
    let mut local = complete_record(1);
    local.title = "Local title".into();
    assert!(matches!(
        document.merge_record_against(&local, other),
        Err(Error::Invalid(_))
    ));
    assert_eq!(document.source(), source);
    assert_eq!(fs::read_to_string(path)?, source);
    assert_eq!(fs::read_to_string(other_path)?, source);
    Ok(())
}

#[test]
fn direct_snapshot_mutations_cannot_delete_move_or_silently_skip_saving() -> Result<(), Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("notes.md");
    let source = three_record_source();
    fs::write(&path, &source)?;
    let mut document = Document::load(&path)?;
    document.records.reverse();
    assert!(matches!(document.delete_record(9), Err(Error::Invalid(_))));
    assert!(matches!(document.move_record(9, 0), Err(Error::Invalid(_))));
    assert!(matches!(document.move_record(9, 2), Err(Error::Invalid(_))));
    assert_eq!(fs::read_to_string(&path)?, source);
    let mut document = Document::load(&path)?;
    document.records[0].title = "Direct mutation".into();
    let local = document.records[0].clone();
    assert!(matches!(
        document.save_record(&local),
        Err(Error::Invalid(_))
    ));
    assert!(matches!(
        document.merge_record(&local),
        Err(Error::Invalid(_))
    ));
    assert_eq!(fs::read_to_string(&path)?, source);
    assert_eq!(document.source(), source);
    Ok(())
}
