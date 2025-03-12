// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_empty_first_string() {
    assert_eq!(osa_distance("", "bca"), 3);
}

#[test]
fn test_osa_distance_empty_second_string() {
    assert_eq!(osa_distance("ab", ""), 2);
}

#[test]
fn test_osa_distance_single_character_different() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_single_character_same() {
    assert_eq!(osa_distance("a", "a"), 0);
}

#[test]
fn test_osa_distance_two_characters_different() {
    assert_eq!(osa_distance("ab", "cd"), 2);
}

#[test]
fn test_osa_distance_two_characters_same() {
    assert_eq!(osa_distance("ab", "ab"), 0);
}

#[test]
fn test_osa_distance_two_characters_one_same_one_diff() {
    assert_eq!(osa_distance("ab", "ac"), 1);
}

