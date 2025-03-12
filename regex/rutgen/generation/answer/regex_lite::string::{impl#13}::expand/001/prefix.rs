// Answer 0

#[test]
fn test_expand_valid_unbraced_capture_references() {
    struct TestCaptures<'h> {
        haystack: &'h str,
        slots: CaptureLocations,
        pikevm: Arc<PikeVM>,
    }

    let haystack = "On 14-03-2010, I became a Tennessee lamb.";
    let slots = CaptureLocations(vec![Some(0), Some(3), Some(6)]);
    let pikevm = Arc::new(PikeVM { nfa: NFA::default() });
    let caps = TestCaptures { haystack, slots, pikevm };

    let mut dst = String::new();
    caps.expand("$2-$1", &mut dst);
}

#[test]
fn test_expand_valid_braced_capture_references() {
    struct TestCaptures<'h> {
        haystack: &'h str,
        slots: CaptureLocations,
        pikevm: Arc<PikeVM>,
    }

    let haystack = "On 14-03-2010, I became a Tennessee lamb.";
    let slots = CaptureLocations(vec![Some(0), Some(3), Some(6)]);
    let pikevm = Arc::new(PikeVM { nfa: NFA::default() });
    let caps = TestCaptures { haystack, slots, pikevm };

    let mut dst = String::new();
    caps.expand("${year}-${month}-${day}", &mut dst);
}

#[test]
fn test_expand_mixed_references() {
    struct TestCaptures<'h> {
        haystack: &'h str,
        slots: CaptureLocations,
        pikevm: Arc<PikeVM>,
    }

    let haystack = "On 14-03-2010, I became a Tennessee lamb.";
    let slots = CaptureLocations(vec![Some(0), None, Some(6)]);
    let pikevm = Arc::new(PikeVM { nfa: NFA::default() });
    let caps = TestCaptures { haystack, slots, pikevm };

    let mut dst = String::new();
    caps.expand("$1-${nonexistent}-$3", &mut dst);
}

#[test]
fn test_expand_empty_replacement() {
    struct TestCaptures<'h> {
        haystack: &'h str,
        slots: CaptureLocations,
        pikevm: Arc<PikeVM>,
    }

    let haystack = "On 14-03-2010, I became a Tennessee lamb.";
    let slots = CaptureLocations(vec![Some(0), Some(3), Some(6)]);
    let pikevm = Arc::new(PikeVM { nfa: NFA::default() });
    let caps = TestCaptures { haystack, slots, pikevm };

    let mut dst = String::new();
    caps.expand("", &mut dst);
}

#[test]
fn test_expand_literal_dollar_signs() {
    struct TestCaptures<'h> {
        haystack: &'h str,
        slots: CaptureLocations,
        pikevm: Arc<PikeVM>,
    }

    let haystack = "On 14-03-2010, I became a Tennessee lamb.";
    let slots = CaptureLocations(vec![Some(0), Some(3), Some(6)]);
    let pikevm = Arc::new(PikeVM { nfa: NFA::default() });
    let caps = TestCaptures { haystack, slots, pikevm };

    let mut dst = String::new();
    caps.expand("$$ $1 $$", &mut dst);
}

