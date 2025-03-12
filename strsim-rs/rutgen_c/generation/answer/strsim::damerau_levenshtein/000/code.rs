// Answer 0

#[test]
fn test_damerau_levenshtein_basic() {
    assert_eq!(2, damerau_levenshtein("ab", "bca"));
}

#[test]
fn test_damerau_levenshtein_empty_strings() {
    assert_eq!(3, damerau_levenshtein("", "abc"));
    assert_eq!(3, damerau_levenshtein("abc", ""));
    assert_eq!(0, damerau_levenshtein("", ""));
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    assert_eq!(0, damerau_levenshtein("test", "test"));
}

#[test]
fn test_damerau_levenshtein_single_character() {
    assert_eq!(1, damerau_levenshtein("a", "b"));
    assert_eq!(1, damerau_levenshtein("a", ""));
    assert_eq!(1, damerau_levenshtein("", "a"));
}

#[test]
fn test_damerau_levenshtein_difference_in_length() {
    assert_eq!(2, damerau_levenshtein("abc", "abcdef"));
    assert_eq!(3, damerau_levenshtein("abcdef", "abc"));
}

#[test]
fn test_damerau_levenshtein_multiple_operations() {
    assert_eq!(4, damerau_levenshtein("kitten", "sitting"));
    assert_eq!(3, damerau_levenshtein("flaw", "lawn"));
}

#[test]
fn test_damerau_levenshtein_case_sensitivity() {
    assert_eq!(1, damerau_levenshtein("abc", "ABC"));
}

