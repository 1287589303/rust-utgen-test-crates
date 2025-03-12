// Answer 0

#[test]
fn test_damerau_levenshtein_diff_chars() {
    let s1 = "abcde".chars();
    let s2 = "fghij".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 10);
}

#[test]
fn test_damerau_levenshtein_one_char_diff() {
    let s1 = "a".chars();
    let s2 = "b".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 2);
}

#[test]
fn test_damerau_levenshtein_empty_and_non_empty() {
    let s1 = "".chars();
    let s2 = "abc".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_two_distinct_chars() {
    let s1 = "hello".chars();
    let s2 = "world".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 8);
}

#[test]
fn test_damerau_levenshtein_large_distances() {
    let s1 = "abcdefghij".chars();
    let s2 = "klmnopqrst".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 20);
}

