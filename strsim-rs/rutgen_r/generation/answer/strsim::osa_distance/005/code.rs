// Answer 0

#[test]
fn test_osa_distance_non_matching_adjacent_transposition() {
    assert_eq!(osa_distance("abc", "bca"), 3);
}

#[test]
fn test_osa_distance_empty_first_string() {
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_empty_second_string() {
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_single_character_difference() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_two_characters_with_transposition() {
    assert_eq!(osa_distance("ab", "ba"), 1);
}

