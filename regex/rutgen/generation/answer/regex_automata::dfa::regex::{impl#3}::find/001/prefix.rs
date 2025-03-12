// Answer 0

#[test]
fn test_find_matching_pattern() {
    struct DummyAutomaton;
    
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"zzzfoo12345zzz",
        span: Span::new(0, 15),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input);
}

#[test]
fn test_find_multiple_patterns() {
    struct DummyAutomaton;

    let re = Regex::new("abc|a").unwrap();
    let input = Input {
        haystack: b"abc",
        span: Span::new(0, 3),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input);
}

#[test]
fn test_find_no_match() {
    struct DummyAutomaton;

    let re = Regex::new("xyz").unwrap();
    let input = Input {
        haystack: b"abc",
        span: Span::new(0, 3),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input);
}

#[test]
#[should_panic]
fn test_find_panic_due_to_configuration() {
    struct DummyAutomaton;

    let re = Regex::new(".+").unwrap();
    let input = Input {
        haystack: b"\xFF",
        span: Span::new(0, 1),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input);
}

#[test]
fn test_find_empty_input() {
    struct DummyAutomaton;

    let re = Regex::new("foo").unwrap();
    let input = Input {
        haystack: b"",
        span: Span::new(0, 0),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input);
}

#[test]
fn test_find_with_anchors() {
    struct DummyAutomaton;

    let re = Regex::new(r"^foo").unwrap();
    let input = Input {
        haystack: b"foobar",
        span: Span::new(0, 6),
        anchored: Anchored::AnchorStart,
        earliest: true,
    };
    re.find(input);
}

#[test]
fn test_find_boundary_cases() {
    struct DummyAutomaton;

    let re = Regex::new("a{2,3}").unwrap();
    let input = Input {
        haystack: b"aa",
        span: Span::new(0, 2),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input);

    let input2 = Input {
        haystack: b"aaa",
        span: Span::new(0, 3),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input2);
    
    let input3 = Input {
        haystack: b"a",
        span: Span::new(0, 1),
        anchored: Anchored::Unanchored,
        earliest: true,
    };
    re.find(input3);
}

