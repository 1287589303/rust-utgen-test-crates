// Answer 0

#[test]
fn test_new_with_non_empty_hirs() {
    let config = Config {
        match_kind: Some(MatchKind::Anchor),
        utf8_empty: Some(true),
        autopre: None,
        pre: None,
        which_captures: None,
        nfa_size_limit: None,
        onepass_size_limit: None,
        hybrid_cache_capacity: None,
        hybrid: Some(false),
        dfa: Some(true),
        dfa_size_limit: None,
        dfa_state_limit: None,
        onepass: None,
        backtrack: None,
        byte_classes: None,
        line_terminator: None,
    };

    struct DummyHir;

    impl DummyHir {
        fn properties(&self) -> hir::Properties {
            hir::Properties::default()
        }
    }

    let hir1 = DummyHir;
    let hir2 = DummyHir;
    let hirs: Vec<&DummyHir> = vec![&hir1, &hir2];
    
    let regex_info = RegexInfo::new(config, &hirs);
}

#[test]
fn test_new_with_empty_hirs() {
    let config = Config {
        match_kind: None,
        utf8_empty: None,
        autopre: Some(false),
        pre: None,
        which_captures: None,
        nfa_size_limit: None,
        onepass_size_limit: None,
        hybrid_cache_capacity: None,
        hybrid: Some(true),
        dfa: None,
        dfa_size_limit: None,
        dfa_state_limit: None,
        onepass: Some(false),
        backtrack: None,
        byte_classes: None,
        line_terminator: Some(b'\n'),
    };

    let hirs: Vec<&DummyHir> = vec![];
    
    let regex_info = RegexInfo::new(config, &hirs);
}

