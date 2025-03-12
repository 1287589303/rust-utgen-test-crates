// Answer 0

#[test]
fn test_build_valid_non_empty_pattern() {
    let mut builder = Builder::new();
    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };
    builder.configure(config);
    let _ = builder.build("abc");
}

#[test]
fn test_build_empty_pattern() {
    let mut builder = Builder::new();
    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };
    builder.configure(config);
    let _ = builder.build("");
}

#[test]
fn test_build_pattern_with_special_characters() {
    let mut builder = Builder::new();
    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };
    builder.configure(config);
    let _ = builder.build("a.*b");
}

#[test]
fn test_build_malformed_pattern() {
    let mut builder = Builder::new();
    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };
    builder.configure(config);
    let result = builder.build("[a-z");
    assert!(result.is_err());
}

#[test]
fn test_build_edge_case_patterns() {
    let mut builder = Builder::new();
    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };
    builder.configure(config);
    
    let _ = builder.build("^$");
    let _ = builder.build(".*");
    let _ = builder.build("a|b");
}

#[test]
fn test_build_pattern_with_different_config_options() {
    let mut builder = Builder::new();
    
    let config1 = Config {
        match_kind: Some(MatchKind::All),
        unicode: true,
        ..Default::default()
    };
    builder.configure(config1);
    let _ = builder.build("a");
    
    let config2 = Config {
        match_kind: Some(MatchKind::All),
        unicode: false,
        ..Default::default()
    };
    builder.configure(config2);
    let _ = builder.build("b");
}

