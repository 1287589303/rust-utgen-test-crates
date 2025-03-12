// Answer 0

#[test]
fn test_hybrid_try_search_half_rev_quadratic_error() {
    let dfa = crate::hybrid::dfa::DFA {
        config: crate::hybrid::dfa::Config::default(),
        nfa: crate::thompson::NFA::default(),
        stride2: 0,
        start_map: crate::hybrid::dfa::StartByteMap::default(),
        classes: crate::hybrid::dfa::ByteClasses::default(),
        quitset: crate::hybrid::dfa::ByteSet::default(),
        cache_capacity: 0,
    };
    
    let mut cache = crate::hybrid::dfa::Cache::default();
    
    let haystack: &[u8] = b"test input";
    let input = crate::Input::new(&haystack)
        .span(crate::Span::new(0, haystack.len()))
        .earliest(true);
    
    let min_start = 0; // value < input.end() - 1
    
    let _ = crate::hybrid::hybrid_try_search_half_rev(&dfa, &mut cache, &input, min_start);
}

