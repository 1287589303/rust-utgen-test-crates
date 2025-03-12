// Answer 0

#[test]
fn test_new_valid_regex() {
    let pattern = "foo[0-9]+bar";
    let _ = DFA::new(pattern).unwrap();
}

#[test]
fn test_new_empty_string() {
    let pattern = "";
    let result = DFA::new(pattern);
    assert!(result.is_err());
}

#[test]
fn test_new_single_character() {
    let pattern = "a";
    let _ = DFA::new(pattern).unwrap();
}

#[test]
fn test_new_special_character() {
    let pattern = ".*";
    let _ = DFA::new(pattern).unwrap();
}

#[test]
fn test_new_digit_pattern() {
    let pattern = "^\\d+$";
    let _ = DFA::new(pattern).unwrap();
}

#[test]
fn test_new_invalid_pattern() {
    let pattern = "[a-z";
    let result = DFA::new(pattern);
    assert!(result.is_err());
}

#[test]
fn test_new_pattern_with_quantifiers() {
    let pattern = "foo[0-9]{1,3}bar";
    let _ = DFA::new(pattern).unwrap();
}

