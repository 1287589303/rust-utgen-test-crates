// Answer 0

#[test]
fn test_which_overlapping_imp_case1() {
    let haystack = b"abc";
    let input = Input::new(&haystack)
        .set_start(0)
        .set_end(0);
    let mut patset = PatternSet::new(10);
    
    let mut pikevm = PikeVM {
        config: Config::default().match_kind(MatchKind::All),
        nfa: NFA::default(),
    };
    
    let mut cache = Cache::new(&pikevm);
    
    pikevm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_case2() {
    let haystack = b"def";
    let input = Input::new(&haystack)
        .set_start(0)
        .set_end(0);
    let mut patset = PatternSet::new(10);
    
    let mut pikevm = PikeVM {
        config: Config::default().match_kind(MatchKind::All),
        nfa: NFA::default(),
    };

    let mut cache = Cache::new(&pikevm);
    
    pikevm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_empty_pattern_set() {
    let haystack = b"xyz";
    let input = Input::new(&haystack)
        .set_start(0)
        .set_end(0);
    let mut patset = PatternSet::new(0);
    
    let mut pikevm = PikeVM {
        config: Config::default().match_kind(MatchKind::All),
        nfa: NFA::default(),
    };

    let mut cache = Cache::new(&pikevm);
    
    pikevm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_full_patset() {
    let haystack = b"ghi";
    let input = Input::new(&haystack)
        .set_start(0)
        .set_end(0);
    let mut patset = PatternSet::new(1);
    
    let mut pikevm = PikeVM {
        config: Config::default().match_kind(MatchKind::All),
        nfa: NFA::default(),
    };

    let mut cache = Cache::new(&pikevm);
    
    let _ = patset.try_insert(PatternID(0));
    pikevm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

