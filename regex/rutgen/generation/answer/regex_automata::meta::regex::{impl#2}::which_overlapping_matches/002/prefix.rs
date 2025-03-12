// Answer 0

#[test]
fn test_which_overlapping_matches_empty_haystack() {
    let patterns = &["\\w+", "\\d+", "foo", "bar"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(&[]);
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_single_char_haystack() {
    let patterns = &["a", "b", "c"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"a");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_minlen_haystack() {
    let patterns = &["\\w+"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"abc");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_minlen_plus_one_haystack() {
    let patterns = &["\\w{3}"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"abcd");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_anchored_true() {
    let patterns = &["^foo"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"foobar");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_anchored_false() {
    let patterns = &["foo$"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"barfoo");
    let mut patset = PatternSet::new(re.pattern_len());
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_capacity_zero() {
    let patterns = &["\\d+"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"123");
    let mut patset = PatternSet::new(0);
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_capacity_one() {
    let patterns = &["\\w+"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"abc");
    let mut patset = PatternSet::new(1);
    re.which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_capacity_two() {
    let patterns = &["\\d+", "\\w+"];
    let re = Regex::builder()
        .configure(Regex::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();

    let input = Input::new(b"123abc");
    let mut patset = PatternSet::new(2);
    re.which_overlapping_matches(&input, &mut patset);
}

