// Answer 0

#[test]
fn test_damerau_levenshtein_different_single_chars() {
    let s1: Vec<char> = "a".chars().collect();
    let s2: Vec<char> = "b".chars().collect();
    damerau_levenshtein_impl(s1.into_iter(), s1.len(), s2.into_iter(), s2.len());
}

#[test]
fn test_damerau_levenshtein_longer_dissimilar_strings() {
    let s1: Vec<char> = "abcdefghij".chars().collect();
    let s2: Vec<char> = "klmnopqrst".chars().collect();
    damerau_levenshtein_impl(s1.into_iter(), s1.len(), s2.into_iter(), s2.len());
}

#[test]
fn test_damerau_levenshtein_longer_strings_with_differences() {
    let s1: Vec<char> = "hello".chars().collect();
    let s2: Vec<char> = "world".chars().collect();
    damerau_levenshtein_impl(s1.into_iter(), s1.len(), s2.into_iter(), s2.len());
}

#[test]
fn test_damerau_levenshtein_strings_with_common_prefix() {
    let s1: Vec<char> = "prefix".chars().collect();
    let s2: Vec<char> = "pretense".chars().collect();
    damerau_levenshtein_impl(s1.into_iter(), s1.len(), s2.into_iter(), s2.len());
}

#[test]
fn test_damerau_levenshtein_edge_case_empty_overlapping() {
    let s1: Vec<char> = "abc".chars().collect();
    let s2: Vec<char> = "a".chars().collect();
    damerau_levenshtein_impl(s1.into_iter(), s1.len(), s2.into_iter(), s2.len());
}

