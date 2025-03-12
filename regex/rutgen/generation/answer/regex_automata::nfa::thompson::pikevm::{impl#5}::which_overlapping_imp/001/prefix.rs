// Answer 0

#[test]
fn test_which_overlapping_imp_empty_input_done() {
    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA(Arc::new(Default::default())),
    };
    let input = Input::new(&[]).span(0..0);
    let mut cache = Cache::new(&pike_vm);
    let mut patset = PatternSet::new(0);

    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_zero_length_haystack_done() {
    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA(Arc::new(Default::default())),
    };
    let input = Input::new(&[]).span(0..0);
    let mut cache = Cache::new(&pike_vm);
    let mut patset = PatternSet::new(0);

    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

