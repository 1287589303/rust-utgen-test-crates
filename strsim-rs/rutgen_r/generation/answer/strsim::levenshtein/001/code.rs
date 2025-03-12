// Answer 0

#[test]
fn test_levenshtein_equal_strings() {
    assert_eq!(levenshtein("hello", "hello"), 0);
}

#[test]
fn test_levenshtein_single_substitution() {
    assert_eq!(levenshtein("cat", "bat"), 1);
}

#[test]
fn test_levenshtein_single_insertion() {
    assert_eq!(levenshtein("abc", "abdc"), 1);
}

#[test]
fn test_levenshtein_single_deletion() {
    assert_eq!(levenshtein("abc", "ac"), 1);
}

#[test]
fn test_levenshtein_multiple_operations() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}

#[test]
fn test_levenshtein_empty_strings() {
    assert_eq!(levenshtein("", ""), 0);
}

#[test]
fn test_levenshtein_one_empty_string() {
    assert_eq!(levenshtein("hello", ""), 5);
    assert_eq!(levenshtein("", "hello"), 5);
}

#[test]
fn test_levenshtein_longer_strings() {
    assert_eq!(levenshtein("horse", "ros"), 3);
}

#[test]
fn test_levenshtein_identical_strings_of_different_lengths() {
    assert_eq!(levenshtein("abc", "abcd"), 1);
    assert_eq!(levenshtein("abcd", "abc"), 1);
}

