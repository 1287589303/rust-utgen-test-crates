// Answer 0

#[test]
fn test_index_valid_case_0() {
    let slots = CaptureLocations(vec![Some(NonMaxUsize::new(0).unwrap())]);
    let pikevm = Arc::new(PikeVM { nfa: NFA {} });
    let captures = Captures {
        haystack: "test",
        slots,
        pikevm,
    };
    let _result = captures.index(0);
}

#[test]
fn test_index_valid_case_length() {
    let slots = CaptureLocations(vec![Some(NonMaxUsize::new(0).unwrap())]);
    let pikevm = Arc::new(PikeVM { nfa: NFA {} });
    let captures = Captures {
        haystack: "test",
        slots,
        pikevm,
    };
    let _result = captures.index(captures.len() - 1);
}

#[should_panic(expected = "no group at index '1'")]
#[test]
fn test_index_out_of_bounds() {
    let slots = CaptureLocations(vec![Some(NonMaxUsize::new(0).unwrap())]);
    let pikevm = Arc::new(PikeVM { nfa: NFA {} });
    let captures = Captures {
        haystack: "test",
        slots,
        pikevm,
    };
    let _result = captures.index(1);
}

