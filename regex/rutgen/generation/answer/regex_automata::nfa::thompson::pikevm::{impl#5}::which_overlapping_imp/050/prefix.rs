// Answer 0

#[test]
fn test_which_overlapping_imp_case1() {
    let config = Config::new().match_kind(MatchKind::All);
    let pike_vm = PikeVM { config, nfa: NFA::default() };
    let haystack = b"samwise";
    let input = Input::new(&haystack).span(0..0).anchored(Anchored::No);
    let mut cache = Cache::new(&pike_vm);
    let mut patset = PatternSet::new(10);
    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_case2() {
    let config = Config::new().match_kind(MatchKind::All);
    let pike_vm = PikeVM { config, nfa: NFA::default() };
    let haystack = b"hello world";
    let input = Input::new(&haystack).span(0..0).anchored(Anchored::No);
    let mut cache = Cache::new(&pike_vm);
    let mut patset = PatternSet::new(10);
    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_case3() {
    let config = Config::new().match_kind(MatchKind::All);
    let pike_vm = PikeVM { config, nfa: NFA::default() };
    let haystack = b"abracadabra";
    let input = Input::new(&haystack).span(0..0).anchored(Anchored::No);
    let mut cache = Cache::new(&pike_vm);
    let mut patset = PatternSet::new(10);
    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_case4() {
    let config = Config::new().match_kind(MatchKind::All);
    let pike_vm = PikeVM { config, nfa: NFA::default() };
    let haystack = b"abcabcabc";
    let input = Input::new(&haystack).span(0..0).anchored(Anchored::No);
    let mut cache = Cache::new(&pike_vm);
    let mut patset = PatternSet::new(10);
    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_case5() {
    let config = Config::new().match_kind(MatchKind::All);
    let pike_vm = PikeVM { config, nfa: NFA::default() };
    let haystack = b"regex";
    let input = Input::new(&haystack).span(0..0).anchored(Anchored::No);
    let mut cache = Cache::new(&pike_vm);
    let mut patset = PatternSet::new(10);
    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

