// Answer 0

#[test]
fn test_thompson_config_utf8_true() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        utf8: Some(true),
        reverse: None,
        nfa_size_limit: Some(Some(5000)),
        shrink: None,
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_config_utf8_false_reverse_true() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        utf8: Some(false),
        reverse: Some(true),
        nfa_size_limit: None,
        shrink: Some(true),
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_config_nfa_size_limit() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        utf8: Some(true),
        reverse: None,
        nfa_size_limit: Some(Some(10000)),
        shrink: Some(false),
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_config_shrink_true() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        utf8: None,
        reverse: Some(false),
        nfa_size_limit: Some(Some(0)),
        shrink: Some(true),
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_config_reverse_false() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        utf8: Some(false),
        reverse: Some(false),
        nfa_size_limit: Some(Some(2500)),
        shrink: None,
    };
    builder.thompson(config);
}

