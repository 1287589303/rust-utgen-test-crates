// Answer 0

#[test]
fn test_dfa_find_multiple_patterns_anchored_no() {
    let re = DFA::new("foo|bar|baz")?;
    let mut cache = re.create_cache();
    let input = Input::new("barbaz12345")
        .anchored(Anchored::No);
    let expected = Match::must(1, 0..3);
    let result = re.find(&mut cache, input);
}

#[test]
fn test_dfa_find_multiple_patterns_with_different_input() {
    let re = DFA::new("quick|brown|fox")?;
    let mut cache = re.create_cache();
    let input = Input::new("the quick brown fox jumps over the lazy dog")
        .anchored(Anchored::No);
    let expected = Match::must(0, 4..9);
    let result = re.find(&mut cache, input);
}

#[test]
fn test_dfa_find_multiple_patterns_with_no_matches() {
    let re = DFA::new("cat|dog|fish")?;
    let mut cache = re.create_cache();
    let input = Input::new("the bird flies")
        .anchored(Anchored::No);
    let result = re.find(&mut cache, input);
}

#[test]
fn test_dfa_find_with_cache_state() {
    let re = DFA::new("apple|banana|grape")?;
    let mut cache = re.create_cache();
    let input = Input::new("banana split with apple pie")
        .anchored(Anchored::No);
    let expected = Match::must(1, 0..6);
    let result = re.find(&mut cache, input);
}

