// Answer 0

#[test]
fn test_levenshtein_identical_strings() {
    assert_eq!(levenshtein("apple", "apple"), 0);
}

#[test]
fn test_levenshtein_insertions() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}

#[test]
fn test_levenshtein_deletions() {
    assert_eq!(levenshtein("sitting", "kitten"), 3);
}

#[test]
fn test_levenshtein_substitutions() {
    assert_eq!(levenshtein("flaw", "lawn"), 2);
}

#[test]
fn test_levenshtein_empty_strings() {
    assert_eq!(levenshtein("", ""), 0);
}

#[test]
fn test_levenshtein_empty_first_string() {
    assert_eq!(levenshtein("", "abc"), 3);
}

#[test]
fn test_levenshtein_empty_second_string() {
    assert_eq!(levenshtein("abc", ""), 3);
}

#[test]
fn test_levenshtein_special_characters() {
    assert_eq!(levenshtein("123", "1a2b3"), 4);
}

