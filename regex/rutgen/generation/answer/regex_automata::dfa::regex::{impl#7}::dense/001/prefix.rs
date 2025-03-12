// Answer 0

#[test]
fn test_dense_config_all_fields_set() {
    let mut builder = Builder::new();
    let config = dense::Config {
        accelerate: Some(true),
        pre: Some(Some(Prefilter::new())),
        minimize: Some(true),
        match_kind: Some(MatchKind::SomeVariant),
        start_kind: Some(StartKind::SomeVariant),
        starts_for_each_pattern: Some(true),
        byte_classes: Some(true),
        unicode_word_boundary: Some(true),
        quitset: Some(ByteSet::new()),
        specialize_start_states: Some(true),
        dfa_size_limit: Some(Some(10)),
        determinize_size_limit: Some(Some(20)),
    };
    builder.dense(config);
}

#[test]
fn test_dense_config_no_fields_set() {
    let mut builder = Builder::new();
    let config = dense::Config::default();
    builder.dense(config);
}

#[test]
fn test_dense_config_partial_fields() {
    let mut builder = Builder::new();
    let config = dense::Config {
        accelerate: Some(false),
        pre: None,
        minimize: Some(false),
        match_kind: None,
        start_kind: None,
        starts_for_each_pattern: Some(false),
        byte_classes: None,
        unicode_word_boundary: None,
        quitset: None,
        specialize_start_states: Some(false),
        dfa_size_limit: Some(Some(0)),
        determinize_size_limit: None,
    };
    builder.dense(config);
}

#[test]
fn test_dense_config_different_size_limits() {
    let mut builder = Builder::new();
    let config_small_limit = dense::Config {
        accelerate: Some(true),
        pre: None,
        minimize: Some(true),
        match_kind: None,
        start_kind: None,
        starts_for_each_pattern: Some(true),
        byte_classes: Some(true),
        unicode_word_boundary: Some(false),
        quitset: None,
        specialize_start_states: Some(false),
        dfa_size_limit: Some(Some(0)),
        determinize_size_limit: None,
    };
    builder.dense(config_small_limit);

    let config_large_limit = dense::Config {
        accelerate: Some(true),
        pre: Some(Some(Prefilter::new())),
        minimize: Some(true),
        match_kind: Some(MatchKind::SomeVariant),
        start_kind: Some(StartKind::SomeVariant),
        starts_for_each_pattern: Some(true),
        byte_classes: Some(true),
        unicode_word_boundary: Some(true),
        quitset: Some(ByteSet::new()),
        specialize_start_states: Some(true),
        dfa_size_limit: Some(Some(100)),
        determinize_size_limit: Some(Some(50)),
    };
    builder.dense(config_large_limit);
}

#[test]
fn test_dense_config_with_string_values() {
    let mut builder = Builder::new();
    let config = dense::Config {
        accelerate: Some(true),
        pre: Some(Some(Prefilter::new())),
        minimize: Some(true),
        match_kind: Some(MatchKind::SomeStringVariant("test".to_string())),
        start_kind: Some(StartKind::SomeStringVariant("test_start".to_string())),
        starts_for_each_pattern: Some(false),
        byte_classes: Some(true),
        unicode_word_boundary: Some(false),
        quitset: None,
        specialize_start_states: Some(true),
        dfa_size_limit: Some(Some(1)),
        determinize_size_limit: None,
    };
    builder.dense(config);
}

