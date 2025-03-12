// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_first_string_empty() {
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_second_string_empty() {
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_single_character_difference() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_single_character_same() {
    assert_eq!(osa_distance("a", "a"), 0);
}

#[test]
fn test_osa_distance_two_character_difference() {
    assert_eq!(osa_distance("ab", "cd"), 2);
}

#[test]
fn test_osa_distance_two_character_same() {
    assert_eq!(osa_distance("ab", "ab"), 0);
}

#[test]
fn test_osa_distance_two_character_transposition() {
    assert_eq!(osa_distance("ab", "ba"), 1);
}

#[test]
fn test_osa_distance_identical_strings() {
    assert_eq!(osa_distance("hello", "hello"), 0);
}

#[test]
fn test_osa_distance_substring_transposition() {
    assert_eq!(osa_distance("abc", "acb"), 1);
}

