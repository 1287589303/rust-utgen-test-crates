// Answer 0

#[test]
fn test_search_imp_input_done_empty_haystack() {
    let input = Input::new(&[]).span(Span { start: 0, end: 0 });
    let config = Config::new();
    let nfa = NFA(Default::default());
    let pikevm = PikeVM { config, nfa };
    let mut cache = Cache::new(&pikevm);
    let mut slots: Vec<Option<NonMaxUsize>> = Vec::new();
    let result = pikevm.search_imp(&mut cache, &input, &mut slots);
}

#[test]
fn test_search_imp_input_done_empty_haystack_nonempty_slots() {
    let input = Input::new(&[]).span(Span { start: 0, end: 0 });
    let config = Config::new();
    let nfa = NFA(Default::default());
    let pikevm = PikeVM { config, nfa };
    let mut cache = Cache::new(&pikevm);
    let mut slots: Vec<Option<NonMaxUsize>> = vec![None; 5];
    let result = pikevm.search_imp(&mut cache, &input, &mut slots);
}

