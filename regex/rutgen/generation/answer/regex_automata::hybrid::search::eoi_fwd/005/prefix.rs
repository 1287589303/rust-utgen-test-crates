// Answer 0

#[test]
fn test_eoi_fwd_empty_haystack() {
    let haystack: &[u8] = &[];
    let span = Span { start: 0, end: 0 };
    let input = Input::new(&haystack).span(span);
    let mut sid = LazyStateID::new_unchecked(0);
    let mut mat: Option<HalfMatch> = None;

    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };
    let mut cache = Cache::default();

    let result = eoi_fwd(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

#[test]
fn test_eoi_fwd_haystack_shorter_than_end() {
    let haystack: &[u8] = &[1, 2, 3];
    let span = Span { start: 0, end: 4 };
    let input = Input::new(&haystack).span(span);
    let mut sid = LazyStateID::new_unchecked(1);
    let mut mat: Option<HalfMatch> = None;

    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };
    let mut cache = Cache::default();

    let result = eoi_fwd(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

#[test]
fn test_eoi_fwd_haystack_at_boundary() {
    let haystack: &[u8] = &[4];
    let span = Span { start: 0, end: 1 };
    let input = Input::new(&haystack).span(span);
    let mut sid = LazyStateID::new_unchecked(2);
    let mut mat: Option<HalfMatch> = None;

    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };
    let mut cache = Cache::default();

    let result = eoi_fwd(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

