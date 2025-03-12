// Answer 0

#[test]
fn test_try_which_overlapping_matches_case_one() {
    let patterns = &[r"\w+", r"\d+", r"foo", r"bar", r"foo1bar"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    let input = Input::new("foo1bar");
    let mut patset = PatternSet::new(dfa.pattern_len());

    dfa.try_which_overlapping_matches(&mut cache, &input, &mut patset).unwrap();
}

#[test]
fn test_try_which_overlapping_matches_case_two() {
    let patterns = &[r"foo", r"bar", r"foobar", r"baz"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    let input = Input::new("foobar");
    let mut patset = PatternSet::new(dfa.pattern_len());

    dfa.try_which_overlapping_matches(&mut cache, &input, &mut patset).unwrap();
}

#[test]
fn test_try_which_overlapping_matches_full() {
    let patterns = &[r"\w+", r"\s+", r"bar", r"f", r"o", r"b", r"baz"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let mut cache = dfa.create_cache();
    let input = Input::new("foobarbaz");
    let mut patset = PatternSet::new(dfa.pattern_len());

    dfa.try_which_overlapping_matches(&mut cache, &input, &mut patset).unwrap();
}

#[test]
fn test_try_which_overlapping_matches_partial() {
    let patterns = &[r"foo", r"bar", r"baz"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    let input = Input::new("foo bar");
    let mut patset = PatternSet::new(dfa.pattern_len());

    dfa.try_which_overlapping_matches(&mut cache, &input, &mut patset).unwrap();
}

