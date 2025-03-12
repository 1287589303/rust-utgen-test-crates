// Answer 0

#[test]
fn test_write_to_len_both() {
    let start_kind = StartKind::Both;
    let _result = start_kind.write_to_len();
}

#[test]
fn test_write_to_len_unanchored() {
    let start_kind = StartKind::Unanchored;
    let _result = start_kind.write_to_len();
}

#[test]
fn test_write_to_len_anchored() {
    let start_kind = StartKind::Anchored;
    let _result = start_kind.write_to_len();
}

