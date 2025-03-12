// Answer 0

#[test]
fn test_levenshtein_identical_strings() {
    assert_eq!(levenshtein("same", "same"), 0);
}

#[test]
fn test_levenshtein_one_insertion() {
    assert_eq!(levenshtein("kitten", "kittens"), 1);
}

#[test]
fn test_levenshtein_one_deletion() {
    assert_eq!(levenshtein("sitting", "kitten"), 3);
}

#[test]
fn test_levenshtein_one_substitution() {
    assert_eq!(levenshtein("kitten", "sittin"), 1);
}

#[test]
fn test_levenshtein_all_operations() {
    assert_eq!(levenshtein("flaw", "lawn"), 2);
}

#[test]
fn test_levenshtein_empty_strings() {
    assert_eq!(levenshtein("", ""), 0);
    assert_eq!(levenshtein("", "nonempty"), 9);
    assert_eq!(levenshtein("nonempty", ""), 9);
}

#[test]
fn test_levenshtein_single_characters() {
    assert_eq!(levenshtein("a", "b"), 1);
    assert_eq!(levenshtein("a", "a"), 0);
    assert_eq!(levenshtein("a", ""), 1);
    assert_eq!(levenshtein("", "a"), 1);
}

