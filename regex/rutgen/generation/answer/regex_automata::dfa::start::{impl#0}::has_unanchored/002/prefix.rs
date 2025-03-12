// Answer 0

#[test]
fn test_has_unanchored_both() {
    let kind = StartKind::Both;
    let result = kind.has_unanchored();
}

#[test]
fn test_has_unanchored_unanchored() {
    let kind = StartKind::Unanchored;
    let result = kind.has_unanchored();
}

