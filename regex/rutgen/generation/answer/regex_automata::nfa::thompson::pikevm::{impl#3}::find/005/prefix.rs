// Answer 0

#[test]
fn test_find_single_pattern_no_matches() {
    let re = PikeVM::new("a")?;
    let mut cache = re.create_cache();
    let input = Input {
        haystack: b"b",
        span: Span { start: 0, end: 1 },
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let result = re.find(&mut cache, input);
}

#[test]
fn test_find_single_pattern_empty_input() {
    let re = PikeVM::new("a")?;
    let mut cache = re.create_cache();
    let input = Input {
        haystack: b"",
        span: Span { start: 0, end: 0 },
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let result = re.find(&mut cache, input);
}

#[test]
fn test_find_single_pattern_non_matching_input() {
    let re = PikeVM::new("x")?;
    let mut cache = re.create_cache();
    let input = Input {
        haystack: b"abc",
        span: Span { start: 0, end: 3 },
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let result = re.find(&mut cache, input);
}

#[test]
fn test_find_single_pattern_with_extra_text() {
    let re = PikeVM::new("z")?;
    let mut cache = re.create_cache();
    let input = Input {
        haystack: b"foo bar",
        span: Span { start: 0, end: 8 },
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let result = re.find(&mut cache, input);
}

#[test]
fn test_find_single_pattern_surrounded_by_non_matching() {
    let re = PikeVM::new("c")?;
    let mut cache = re.create_cache();
    let input = Input {
        haystack: b"abde",
        span: Span { start: 0, end: 4 },
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let result = re.find(&mut cache, input);
}

