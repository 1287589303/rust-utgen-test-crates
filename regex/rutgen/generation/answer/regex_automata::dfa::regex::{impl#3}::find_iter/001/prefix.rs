// Answer 0

#[test]
fn test_find_iter_valid_match() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"foo1 foo12 foo123",
        span: Span::new(0, 17),
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let _matches = re.find_iter(input);
}

#[test]
fn test_find_iter_multiple_matches() {
    let re = Regex::new("abc").unwrap();
    let input = Input {
        haystack: b"abc abc abc",
        span: Span::new(0, 11),
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let _matches = re.find_iter(input);
}

#[test]
fn test_find_iter_pattern_at_start() {
    let re = Regex::new("^foo").unwrap();
    let input = Input {
        haystack: b"foobar",
        span: Span::new(0, 6),
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let _matches = re.find_iter(input);
}

#[test]
fn test_find_iter_pattern_at_end() {
    let re = Regex::new("bar$").unwrap();
    let input = Input {
        haystack: b"foobar",
        span: Span::new(0, 6),
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let _matches = re.find_iter(input);
}

#[test]
fn test_find_iter_empty_haystack() {
    let re = Regex::new("foo").unwrap();
    let input = Input {
        haystack: b"",
        span: Span::new(0, 0),
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let _matches = re.find_iter(input);
} 

#[test]
#[should_panic]
fn test_find_iter_no_match() {
    let re = Regex::new("xyz").unwrap();
    let input = Input {
        haystack: b"foobar",
        span: Span::new(0, 6),
        anchored: Anchored::Unanchored,
        earliest: false,
    };
    let _matches = re.find_iter(input);
}

