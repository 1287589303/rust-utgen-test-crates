// Answer 0

#[test]
fn test_which_overlapping_matches_example_1() {
    use regex_automata::{
        nfa::thompson::pikevm::{PikeVM, Cache},
        Input, MatchKind, PatternSet,
    };

    let patterns = &["\\w+", "\\d+", "foo", "bar"];
    let re = PikeVM::builder()
        .configure(PikeVM::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    let mut cache = re.create_cache();

    let input = Input::new("foo123bar");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_example_2() {
    use regex_automata::{
        nfa::thompson::pikevm::{PikeVM, Cache},
        Input, MatchKind, PatternSet,
    };

    let patterns = &["bar", "baz", "foo"];
    let re = PikeVM::builder()
        .configure(PikeVM::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    let mut cache = re.create_cache();

    let input = Input::new("foobarbaz");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_example_3() {
    use regex_automata::{
        nfa::thompson::pikevm::{PikeVM, Cache},
        Input, MatchKind, PatternSet,
    };

    let patterns = &["\\w+", "notfound", "foo"];
    let re = PikeVM::builder()
        .configure(PikeVM::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    let mut cache = re.create_cache();

    let input = Input::new("hello");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&mut cache, &input, &mut patset);
} 

#[test]
fn test_which_overlapping_matches_example_4() {
    use regex_automata::{
        nfa::thompson::pikevm::{PikeVM, Cache},
        Input, MatchKind, PatternSet,
    };

    let patterns = &["\\w+", "\\d+", "nope", "bar"];
    let re = PikeVM::builder()
        .configure(PikeVM::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    let mut cache = re.create_cache();

    let input = Input::new("1bar2");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&mut cache, &input, &mut patset);
} 

#[test]
fn test_which_overlapping_matches_example_5() {
    use regex_automata::{
        nfa::thompson::pikevm::{PikeVM, Cache},
        Input, MatchKind, PatternSet,
    };

    let patterns = &["pattern1", "pattern2", "notmatchingpattern"];
    let re = PikeVM::builder()
        .configure(PikeVM::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    let mut cache = re.create_cache();

    let input = Input::new("somepattern1text");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&mut cache, &input, &mut patset);
} 

