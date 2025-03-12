// Answer 0

#[test]
fn test_which_overlapping_matches_with_non_empty_haystack() {
    let patterns = &[r"\w+", r"\d+", r"foo"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = re.create_cache();
    let input = Input::new("hello123foo");
    let mut patset = PatternSet::new(re.pattern_len());

    re.which_overlapping_matches_with(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_with_haystack_with_special_chars() {
    let patterns = &[r"\d+", r"hello", r"world"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = re.create_cache();
    let input = Input::new("hello@world123");
    let mut patset = PatternSet::new(re.pattern_len());

    re.which_overlapping_matches_with(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_with_haystack_with_spaces() {
    let patterns = &[r"\s+", r"foo", r"bar"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = re.create_cache();
    let input = Input::new("foo bar");
    let mut patset = PatternSet::new(re.pattern_len());

    re.which_overlapping_matches_with(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_with_haystack_with_minimum_length() {
    let patterns = &[r".{2,}", r"foo"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = re.create_cache();
    let input = Input::new("fo");
    let mut patset = PatternSet::new(re.pattern_len());

    re.which_overlapping_matches_with(&mut cache, &input, &mut patset);
}

