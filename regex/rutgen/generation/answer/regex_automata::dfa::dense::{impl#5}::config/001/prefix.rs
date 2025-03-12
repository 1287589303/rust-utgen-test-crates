// Answer 0

#[test]
fn test_config_default_values() {
    let config = DFA::<&[u32]>::config();
    assert_eq!(config.accelerate, None);
    assert_eq!(config.pre, None);
    assert_eq!(config.minimize, None);
    assert_eq!(config.match_kind, None);
    assert_eq!(config.start_kind, None);
    assert_eq!(config.starts_for_each_pattern, None);
    assert_eq!(config.byte_classes, None);
    assert_eq!(config.unicode_word_boundary, None);
    assert_eq!(config.quitset, None);
    assert_eq!(config.specialize_start_states, None);
    assert_eq!(config.dfa_size_limit, None);
    assert_eq!(config.determinize_size_limit, None);
}

