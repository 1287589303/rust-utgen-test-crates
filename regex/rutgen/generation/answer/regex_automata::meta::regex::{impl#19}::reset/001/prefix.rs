// Answer 0

#[test]
fn test_cache_reset_with_valid_regex() {
    use regex_automata::{meta::Regex, Match, Input};

    let re1 = Regex::new(r"\w").unwrap();
    let re2 = Regex::new(r"\W").unwrap();

    let mut cache = Cache::new(&re1);

    let match_result1 = re1.search_with(&mut cache, &Input::new("Δ"));
    let expected_match1 = Some(Match::must(0, 0..2));
    match_result1;

    cache.reset(&re2);

    let match_result2 = re2.search_with(&mut cache, &Input::new("☃"));
    let expected_match2 = Some(Match::must(0, 0..3));
    match_result2;
}

#[test]
#[should_panic]
fn test_cache_reset_with_invalid_regex() {
    use regex_automata::{meta::Regex, Match, Input};

    let re1 = Regex::new(r"\d").unwrap();
    let re2 = Regex::new(r"\D").unwrap();

    let mut cache = Cache::new(&re1);

    let match_result1 = re1.search_with(&mut cache, &Input::new("123"));
    match_result1;

    cache.reset(&re2);

    let _ = re1.search_with(&mut cache, &Input::new("ABC")); // Should panic or give incorrect result
}

#[test]
fn test_cache_reset_with_matching_and_non_matching() {
    use regex_automata::{meta::Regex, Match, Input};

    let re1 = Regex::new(r"\w").unwrap();
    let re2 = Regex::new(r"\W").unwrap();

    let mut cache = Cache::new(&re1);

    let match_result1 = re1.search_with(&mut cache, &Input::new("a"));
    match_result1;

    cache.reset(&re2);

    let match_result2 = re2.search_with(&mut cache, &Input::new("!"));
    match_result2;

    let match_result3 = re2.search_with(&mut cache, &Input::new("a")); // Non-matching
    match_result3;
}

