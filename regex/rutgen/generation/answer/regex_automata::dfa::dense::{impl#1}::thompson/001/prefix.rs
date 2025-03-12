// Answer 0

#[test]
fn test_thompson_with_reverse_true_and_shrink_true() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        reverse: Some(true),
        shrink: Some(true),
        ..Default::default()
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_with_reverse_true_and_shrink_false() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        reverse: Some(true),
        shrink: Some(false),
        ..Default::default()
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_with_reverse_false_and_shrink_true() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        reverse: Some(false),
        shrink: Some(true),
        ..Default::default()
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_with_reverse_false_and_shrink_false() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        reverse: Some(false),
        shrink: Some(false),
        ..Default::default()
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_with_dfa_size_limit() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        dfa_size_limit: Some(500),
        ..Default::default()
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_with_determinize_size_limit() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        determinize_size_limit: Some(750),
        ..Default::default()
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_with_unicode_true() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        unicode: Some(true),
        ..Default::default()
    };
    builder.thompson(config);
}

#[test]
fn test_thompson_with_unicode_false() {
    let mut builder = Builder::new();
    let config = thompson::Config {
        unicode: Some(false),
        ..Default::default()
    };
    builder.thompson(config);
}

