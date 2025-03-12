// Answer 0

#[test]
fn test_damerau_levenshtein_equal_strings() {
    assert_eq!(0, damerau_levenshtein("abc", "abc"));
}

#[test]
fn test_damerau_levenshtein_single_insertion() {
    assert_eq!(1, damerau_levenshtein("abc", "abci"));
}

#[test]
fn test_damerau_levenshtein_single_deletion() {
    assert_eq!(1, damerau_levenshtein("abci", "abc"));
}

#[test]
fn test_damerau_levenshtein_single_substitution() {
    assert_eq!(1, damerau_levenshtein("abc", "ACC"));
}

#[test]
fn test_damerau_levenshtein_multiple_edits() {
    assert_eq!(2, damerau_levenshtein("abc", "xyz"));
}

#[test]
fn test_damerau_levenshtein_complex_case() {
    assert_eq!(3, damerau_levenshtein("abcdef", "azcedf"));
}

#[test]
fn test_damerau_levenshtein_empty_strings() {
    assert_eq!(3, damerau_levenshtein("", "abc"));
    assert_eq!(3, damerau_levenshtein("abc", ""));
}

