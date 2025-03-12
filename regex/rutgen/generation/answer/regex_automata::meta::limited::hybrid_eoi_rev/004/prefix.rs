// Answer 0

#[test]
fn test_hybrid_eoi_rev_case1() {
    let haystack = b"sample haystack";
    let input = Input::new(&haystack).span(Span { start: 1, end: 5 });
    let mut cache = Cache { trans: vec![LazyStateID::new_unchecked(1)], starts: vec![LazyStateID::new_unchecked(0)], states: vec![], states_to_id: StateMap::default(), sparses: SparseSets::default(), stack: vec![], scratch_state_builder: StateBuilderEmpty::default(), state_saver: StateSaver::default(), memory_usage_state: 0, clear_count: 0, bytes_searched: 0, progress: None };
    let dfa = DFA { config: Config::default(), nfa: thompson::NFA::default(), stride2: 0, start_map: StartByteMap::default(), classes: ByteClasses::default(), quitset: ByteSet::default(), cache_capacity: 0 };
    let mut sid = LazyStateID::new_unchecked(2);
    let mut mat = None;

    let result = hybrid_eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

#[test]
fn test_hybrid_eoi_rev_case2() {
    let haystack = b"test data here";
    let input = Input::new(&haystack).span(Span { start: 2, end: 7 });
    let mut cache = Cache { trans: vec![LazyStateID::new_unchecked(2)], starts: vec![LazyStateID::new_unchecked(1)], states: vec![], states_to_id: StateMap::default(), sparses: SparseSets::default(), stack: vec![], scratch_state_builder: StateBuilderEmpty::default(), state_saver: StateSaver::default(), memory_usage_state: 0, clear_count: 0, bytes_searched: 0, progress: None };
    let dfa = DFA { config: Config::default(), nfa: thompson::NFA::default(), stride2: 0, start_map: StartByteMap::default(), classes: ByteClasses::default(), quitset: ByteSet::default(), cache_capacity: 0 };
    let mut sid = LazyStateID::new_unchecked(2);
    let mut mat = None;

    let result = hybrid_eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

#[test]
fn test_hybrid_eoi_rev_case3() {
    let haystack = b"boundary test";
    let input = Input::new(&haystack).span(Span { start: 3, end: 8 });
    let mut cache = Cache { trans: vec![LazyStateID::new_unchecked(3)], starts: vec![LazyStateID::new_unchecked(1)], states: vec![], states_to_id: StateMap::default(), sparses: SparseSets::default(), stack: vec![], scratch_state_builder: StateBuilderEmpty::default(), state_saver: StateSaver::default(), memory_usage_state: 0, clear_count: 0, bytes_searched: 0, progress: None };
    let dfa = DFA { config: Config::default(), nfa: thompson::NFA::default(), stride2: 0, start_map: StartByteMap::default(), classes: ByteClasses::default(), quitset: ByteSet::default(), cache_capacity: 0 };
    let mut sid = LazyStateID::new_unchecked(4);
    let mut mat = None;

    let result = hybrid_eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

