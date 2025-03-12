// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_first_string_empty() {
    let s1 = "".chars();
    let s2 = "abc".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 3);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_second_string_empty() {
    let s1 = "abc".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 0);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_single_character_insertion() {
    let s1 = "a".chars();
    let s2 = "ab".chars();
    let result = damerau_levenshtein_impl(s1, 1, s2, 2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_character_deletion() {
    let s1 = "ab".chars();
    let s2 = "a".chars();
    let result = damerau_levenshtein_impl(s1, 2, s2, 1);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_character_substitution() {
    let s1 = "a".chars();
    let s2 = "b".chars();
    let result = damerau_levenshtein_impl(s1, 1, s2, 1);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1 = "abc".chars();
    let s2 = "abc".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 3);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_transposition() {
    let s1 = "ab".chars();
    let s2 = "ba".chars();
    let result = damerau_levenshtein_impl(s1, 2, s2, 2);
    assert_eq!(result, 1);
}

