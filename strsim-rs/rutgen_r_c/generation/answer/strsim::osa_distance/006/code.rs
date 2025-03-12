// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_a_empty() {
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_b_empty() {
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_single_char_match() {
    assert_eq!(osa_distance("a", "a"), 0);
}

#[test]
fn test_osa_distance_single_char_different() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_two_chars_different() {
    assert_eq!(osa_distance("ab", "cd"), 2);
}

#[test]
fn test_osa_distance_two_chars_transposition() {
    assert_eq!(osa_distance("ab", "ba"), 1);
}

#[test]
fn test_osa_distance_three_chars_transposition() {
    assert_eq!(osa_distance("abc", "acb"), 1);
}

