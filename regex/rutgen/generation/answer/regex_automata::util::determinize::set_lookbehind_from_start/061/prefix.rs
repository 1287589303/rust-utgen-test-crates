// Answer 0

#[test]
fn test_set_lookbehind_from_start_text_anchor_haystack() {
    let mut builder = StateBuilderMatches(vec![0; 0]);
    let nfa = {
        let lm = LookMatcher::new();
        let mut lookset = LookSet::full();
        lookset.set_insert(Look::Start); // contains_anchor_haystack
        lookset.set_insert(Look::StartLF); // contains_anchor_line
        lookset.set_insert(Look::WordStartHalfAscii); // contains_word
        lookset.set_insert(Look::WordStartHalfUnicode); // contains_word
        let inner = Arc::new(Inner { look_matcher: lm, look_set_any: lookset, reverse: false });
        NFA(inner)
    };
    let start = Start::Text;
    set_lookbehind_from_start(&nfa, &start, &mut builder);
}

#[test]
fn test_set_lookbehind_from_start_text_anchor_haystack_reverse() {
    let mut builder = StateBuilderMatches(vec![0; 0]);
    let nfa = {
        let lm = LookMatcher::new();
        let mut lookset = LookSet::full();
        lookset.set_insert(Look::Start); // contains_anchor_haystack
        lookset.set_insert(Look::StartLF); // contains_anchor_line
        lookset.set_insert(Look::WordStartHalfAscii); // contains_word
        lookset.set_insert(Look::WordStartHalfUnicode); // contains_word
        let inner = Arc::new(Inner { look_matcher: lm, look_set_any: lookset, reverse: true });
        NFA(inner)
    };
    let start = Start::Text;
    set_lookbehind_from_start(&nfa, &start, &mut builder);
}

