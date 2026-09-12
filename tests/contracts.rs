//! Executable contracts for the Premise representation consumed by agents.

use patterns::{
    record::{Field, Fielded, recorded, restored},
    seq::{Keyed, found_ref},
};
use vrdx::document::Record;

#[test]
fn premise_transport_is_lossless_over_identifiers_and_text_boundaries() {
    let identifiers = [
        0,
        1,
        u64::from(u32::MAX),
        9_007_199_254_740_991,
        9_007_199_254_740_992,
        u64::MAX / 2,
        u64::MAX - 1,
        u64::MAX,
    ];
    let texts = [
        "",
        "ASCII",
        "café 日本語",
        "e\u{301} 👩‍💻",
        "a\n\nb",
        "\"\\\t\0",
    ];
    for id in identifiers {
        for text in texts {
            let mut record = Record::new(id);
            record.title = text.into();
            record.status = text.into();
            record.decision = text.into();
            record.context = text.into();
            record.consequences = text.into();
            let shape = recorded(&record);
            let restored: Record = restored(&shape).unwrap();
            assert_eq!(restored, record);
            assert_eq!(recorded(&restored), shape);
            assert_eq!(restored.key(), id);
        }
    }
}

#[test]
fn premise_bulk_transport_and_lookup_keep_identity_after_reordering() {
    let mut records: Vec<_> = [u64::MAX, 0, 42].into_iter().map(Record::new).collect();
    let encoded = records.field();
    assert_eq!(Vec::<Record>::refielded(&encoded), Some(records.clone()));
    records.rotate_left(1);
    for id in [0, 42, u64::MAX] {
        assert_eq!(found_ref(&records, id).unwrap().key(), id);
    }
    assert!(found_ref(&records, 7).is_none());
    if let Field::List(mut rows) = encoded {
        rows.push(Field::Nothing);
        assert!(Vec::<Record>::refielded(&Field::List(rows)).is_none());
    } else {
        panic!("Premise must encode a record sequence as a list");
    }
}

#[test]
fn premise_named_fields_reject_coercion_and_accept_any_field_order() {
    let record = Record::new(u64::MAX);
    let Field::Table(mut rows) = record.field() else {
        panic!("Premise must encode a record as named fields");
    };
    rows.reverse();
    assert_eq!(Record::refielded(&Field::Table(rows.clone())), Some(record));
    for invalid in [
        Field::Whole(-1),
        Field::Real(1.0),
        Field::Flag(true),
        Field::Nothing,
    ] {
        let mut malformed = rows.clone();
        for (name, value) in &mut malformed {
            if name == "id" {
                *value = invalid.clone();
            }
        }
        assert!(Record::refielded(&Field::Table(malformed)).is_none());
    }
}
