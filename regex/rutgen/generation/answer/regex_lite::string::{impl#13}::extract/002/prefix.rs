// Answer 0

#[test]
fn test_extract_with_one_capture() {
    let nfa = NFA {
        pattern: String::from(r"([0-9]{4})"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("year"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let captures = Captures {
        haystack: "2010",
        slots: CaptureLocations(vec![Some(NonMaxUsize::new(0).unwrap())]),
        pikevm: Arc::new(pikevm),
    };

    let (full, [year]) = captures.extract::<1>();
}

#[test]
fn test_extract_with_two_captures() {
    let nfa = NFA {
        pattern: String::from(r"([0-9]{4})-([0-9]{2})"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("year")), Some(Arc::from("month"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let captures = Captures {
        haystack: "2010-03",
        slots: CaptureLocations(vec![Some(NonMaxUsize::new(0).unwrap()), Some(NonMaxUsize::new(1).unwrap())]),
        pikevm: Arc::new(pikevm),
    };

    let (full, [year, month]) = captures.extract::<2>();
}

#[test]
#[should_panic(expected = "asked for 3 groups, but must ask for 2")]
fn test_extract_with_incorrect_capture_count() {
    let nfa = NFA {
        pattern: String::from(r"([0-9]{4})-([0-9]{2})"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("year")), Some(Arc::from("month"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let captures = Captures {
        haystack: "2010-03",
        slots: CaptureLocations(vec![Some(NonMaxUsize::new(0).unwrap()), Some(NonMaxUsize::new(1).unwrap())]),
        pikevm: Arc::new(pikevm),
    };

    let (full, [year, month, day]) = captures.extract::<3>();
}

#[test]
fn test_extract_with_no_captures() {
    let nfa = NFA {
        pattern: String::from(r"Hello"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(0),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let captures = Captures {
        haystack: "Hello",
        slots: CaptureLocations(vec![]),
        pikevm: Arc::new(pikevm),
    };

    let (full, groups): (&str, [&str; 0]) = captures.extract::<0>();
}

