// Answer 0

#[test]
fn test_get_starts_with_none() {
    struct TestConfig {
        start_kind: Option<StartKind>,
    }

    let config = TestConfig { start_kind: None };
    let result = config.start_kind.unwrap_or(StartKind::Both);
}

#[test]
fn test_get_starts_with_both() {
    struct TestConfig {
        start_kind: Option<StartKind>,
    }

    let config = TestConfig { start_kind: Some(StartKind::Both) };
    let result = config.start_kind.unwrap_or(StartKind::Both);
}

#[test]
fn test_get_starts_with_unanchored() {
    struct TestConfig {
        start_kind: Option<StartKind>,
    }

    let config = TestConfig { start_kind: Some(StartKind::Unanchored) };
    let result = config.start_kind.unwrap_or(StartKind::Both);
}

#[test]
fn test_get_starts_with_anchored() {
    struct TestConfig {
        start_kind: Option<StartKind>,
    }

    let config = TestConfig { start_kind: Some(StartKind::Anchored) };
    let result = config.start_kind.unwrap_or(StartKind::Both);
}

