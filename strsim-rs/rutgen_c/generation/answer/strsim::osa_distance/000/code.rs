// Answer 0

#[test]
fn test_osa_distance_basic_case() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_identical_strings() {
    assert_eq!(osa_distance("hello", "hello"), 0);
}

#[test]
fn test_osa_distance_one_empty_string() {
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_both_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_insertions_and_deletions() {
    assert_eq!(osa_distance("abc", "def"), 6);
}

#[test]
fn test_osa_distance_adjacent_transpositions() {
    assert_eq!(osa_distance("abc", "bac"), 1);
}

#[test]
fn test_osa_distance_long_strings() {
    assert_eq!(osa_distance("kitten", "sitting"), 5);
}

