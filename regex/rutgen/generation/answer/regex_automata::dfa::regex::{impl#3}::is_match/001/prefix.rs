// Answer 0

#[test]
fn test_is_match_valid_pattern() {
    struct MockAutomaton;

    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("foo12345bar".as_bytes());
    re.is_match(input);
}

#[test]
fn test_is_match_no_match() {
    struct MockAutomaton;

    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("foobar".as_bytes());
    re.is_match(input);
}

#[test]
fn test_is_match_empty_input() {
    struct MockAutomaton;

    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("".as_bytes());
    re.is_match(input);
}

#[test]
fn test_is_match_boundary_length() {
    struct MockAutomaton;

    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("foo1bar".as_bytes());
    re.is_match(input);
}

#[test]
fn test_is_match_longer_than_pattern() {
    struct MockAutomaton;

    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("foo123456789bar".as_bytes());
    re.is_match(input);
}

#[test]
fn test_is_match_missing_parts() {
    struct MockAutomaton;

    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("12345foo123bar".as_bytes());
    re.is_match(input);
}

#[test]
fn test_is_match_non_matching_chars() {
    struct MockAutomaton;

    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("foobarbaz".as_bytes());
    re.is_match(input);
}

