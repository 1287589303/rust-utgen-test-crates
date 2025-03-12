// Answer 0

#[test]
fn test_start_kind_both() {
    let config = Config::new();
    let updated_config = config.start_kind(StartKind::Both);
}

#[test]
fn test_start_kind_anchored() {
    let config = Config::new();
    let updated_config = config.start_kind(StartKind::Anchored);
}

#[test]
fn test_start_kind_unanchored() {
    let config = Config::new();
    let updated_config = config.start_kind(StartKind::Unanchored);
}

