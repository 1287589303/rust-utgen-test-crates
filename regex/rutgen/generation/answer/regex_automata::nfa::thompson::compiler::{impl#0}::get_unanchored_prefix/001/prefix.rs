// Answer 0

#[test]
fn test_get_unanchored_prefix_none() {
    let config = Config {
        unanchored_prefix: None,
        ..Default::default()
    };
    let result = config.get_unanchored_prefix();
}

#[test]
fn test_get_unanchored_prefix_true() {
    let config = Config {
        unanchored_prefix: Some(true),
        ..Default::default()
    };
    let result = config.get_unanchored_prefix();
}

