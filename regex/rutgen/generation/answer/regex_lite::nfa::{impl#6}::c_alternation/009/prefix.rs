// Answer 0

#[test]
fn test_c_alternation_two_elements() {
    let config = Config { 
        nest_limit: 10, 
        size_limit: None 
    };
    let compiler = Compiler { 
        config, 
        nfa: RefCell::new(NFA { 
            pattern: String::new(), 
            states: vec![], 
            start: 0, 
            is_start_anchored: false, 
            is_match_empty: false, 
            static_explicit_captures_len: None, 
            cap_name_to_index: CaptureNameMap::new(), 
            cap_index_to_name: vec![], 
            memory_extra: 0 
        }) 
    };

    let first = Ok(ThompsonRef { start: 1, end: 2 });
    let second = Ok(ThompsonRef { start: 3, end: 4 });
    let elements = vec![first, second].into_iter();

    let _ = compiler.c_alternation(elements);
}

#[test]
fn test_c_alternation_one_element() {
    let config = Config { 
        nest_limit: 10, 
        size_limit: None 
    };
    let compiler = Compiler { 
        config, 
        nfa: RefCell::new(NFA { 
            pattern: String::new(), 
            states: vec![], 
            start: 0, 
            is_start_anchored: false, 
            is_match_empty: false, 
            static_explicit_captures_len: None, 
            cap_name_to_index: CaptureNameMap::new(), 
            cap_index_to_name: vec![], 
            memory_extra: 0 
        }) 
    };

    let first = Ok(ThompsonRef { start: 1, end: 2 });
    let elements = vec![first].into_iter();

    let _ = compiler.c_alternation(elements);
}

#[test]
#[should_panic]
fn test_c_alternation_no_elements() {
    let config = Config { 
        nest_limit: 10, 
        size_limit: None 
    };
    let compiler = Compiler { 
        config, 
        nfa: RefCell::new(NFA { 
            pattern: String::new(), 
            states: vec![], 
            start: 0, 
            is_start_anchored: false, 
            is_match_empty: false, 
            static_explicit_captures_len: None, 
            cap_name_to_index: CaptureNameMap::new(), 
            cap_index_to_name: vec![], 
            memory_extra: 0 
        }) 
    };

    let elements: Vec<Result<ThompsonRef, Error>> = vec![];
    let _ = compiler.c_alternation(elements.into_iter());
}

