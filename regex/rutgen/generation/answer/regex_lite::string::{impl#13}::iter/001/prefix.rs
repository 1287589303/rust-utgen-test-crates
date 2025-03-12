// Answer 0

#[test]
fn test_iter_with_single_capture_group() {
    let pattern = r"(\w)";
    let nfa = NFA::new(Config::default(), pattern.to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let haystack = "A";

    let slots = vec![None];
    let captures = Captures {
        haystack,
        slots: CaptureLocations(slots.clone()),
        pikevm: Arc::new(pikevm),
    };

    let _it = captures.iter();
}

#[test]
fn test_iter_with_multiple_capture_groups() {
    let pattern = r"(\w)(\d)?(\w)";
    let nfa = NFA::new(Config::default(), pattern.to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let haystack = "A1B";

    let slots = vec![None, None, None];
    let captures = Captures {
        haystack,
        slots: CaptureLocations(slots.clone()),
        pikevm: Arc::new(pikevm),
    };

    let _it = captures.iter();
}

#[test]
fn test_iter_with_empty_capture_group() {
    let pattern = r"(\w)(\d)?(\w)?";
    let nfa = NFA::new(Config::default(), pattern.to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let haystack = "A B";

    let slots = vec![None, None, None, None];
    let captures = Captures {
        haystack,
        slots: CaptureLocations(slots.clone()),
        pikevm: Arc::new(pikevm),
    };

    let _it = captures.iter();
}

#[test]
fn test_iter_with_no_matching_groups() {
    let pattern = r"(\d)(\w)";
    let nfa = NFA::new(Config::default(), pattern.to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let haystack = "AB";

    let slots = vec![None, None];
    let captures = Captures {
        haystack,
        slots: CaptureLocations(slots.clone()),
        pikevm: Arc::new(pikevm),
    };

    let _it = captures.iter();
}

#[test]
fn test_iter_with_alphanumeric_haystack() {
    let pattern = r"(\w+)(\d+)?";
    let nfa = NFA::new(Config::default(), pattern.to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let haystack = "Hello123";

    let slots = vec![None, None];
    let captures = Captures {
        haystack,
        slots: CaptureLocations(slots.clone()),
        pikevm: Arc::new(pikevm),
    };

    let _it = captures.iter();
}

