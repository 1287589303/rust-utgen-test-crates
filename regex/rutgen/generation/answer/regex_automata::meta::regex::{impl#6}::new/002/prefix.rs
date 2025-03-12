// Answer 0

#[test]
fn test_regex_info_new_with_empty_hirs() {
    let config = Config::default();
    let hirs: Vec<&Hir> = vec![];
    let regex_info = RegexInfo::new(config, &hirs);
}

#[test]
fn test_regex_info_new_with_default_config() {
    let config = Config {
        match_kind: None,
        utf8_empty: None,
        autopre: None,
        pre: None,
        which_captures: None,
        nfa_size_limit: None,
        onepass_size_limit: None,
        hybrid_cache_capacity: None,
        hybrid: None,
        dfa: None,
        dfa_size_limit: None,
        dfa_state_limit: None,
        onepass: None,
        backtrack: None,
        byte_classes: None,
        line_terminator: None,
    };
    let hirs: Vec<&Hir> = vec![];
    let regex_info = RegexInfo::new(config, &hirs);
}

