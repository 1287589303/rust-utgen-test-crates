// Answer 0

#[test]
fn test_find_valid_case_1() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"foo123",
        span: Span::new(0, 6),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

#[test]
fn test_find_valid_case_2() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"foo7",
        span: Span::new(0, 4),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

#[test]
fn test_find_valid_case_3() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"foo999",
        span: Span::new(0, 6),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

#[test]
fn test_find_empty_string() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"",
        span: Span::new(0, 0),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

#[test]
fn test_find_no_match() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"bar",
        span: Span::new(0, 3),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

#[test]
fn test_find_special_character() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"foo@3",
        span: Span::new(0, 5),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

#[test]
fn test_find_leading_trailing_whitespace() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let input = Input {
        haystack: b"   foo123   ",
        span: Span::new(0, 12),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

#[test]
fn test_find_large_input() {
    let re = Regex::new("foo[0-9]+").unwrap();
    let haystack = b"f" + &[b'o'; 997] + b"foo123";
    let input = Input {
        haystack,
        span: Span::new(0, haystack.len() as u32),
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = re.find(input);
}

