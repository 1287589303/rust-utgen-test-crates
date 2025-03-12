// Answer 0

#[test]
fn test_set_lookbehind_from_start_word_byte_no_word() {
    use crate::util::Look;
    use alloc::sync::Arc;
    use crate::nfa::thompson::{NFA, LookMatcher};

    let mut builder = StateBuilderMatches(Vec::new());
    let line_terminator = b'\x01'; // non-ASCII word byte
    let look_matcher = LookMatcher::new();
    let nfa = Arc::new(NFA::always_match());

    look_matcher.set_line_terminator(line_terminator);

    nfa.0.look_matcher = look_matcher.clone();
    nfa.0.look_set_any = LookSet::empty(); // ensure contains_word is false

    let start = Start::WordByte;

    set_lookbehind_from_start(&nfa, &start, &mut builder);
}

