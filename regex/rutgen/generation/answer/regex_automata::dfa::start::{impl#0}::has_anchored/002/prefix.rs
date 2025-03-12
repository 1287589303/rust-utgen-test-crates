// Answer 0

#[test]
fn test_has_anchored_both() {
    let kind = StartKind::Both;
    let result = kind.has_anchored();
}

#[test]
fn test_has_anchored_anchored() {
    let kind = StartKind::Anchored;
    let result = kind.has_anchored();
}

