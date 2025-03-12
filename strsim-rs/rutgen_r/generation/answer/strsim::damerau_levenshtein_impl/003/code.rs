// Answer 0

#[test]
fn test_damerau_levenshtein_case_1() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3); // Expected distance for "kitten" to "sitting"
}

#[test]
fn test_damerau_levenshtein_case_2() {
    let s1 = "flaw".chars();
    let s2 = "lawn".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 2); // Expected distance for "flaw" to "lawn"
}

#[test]
fn test_damerau_levenshtein_case_3() {
    let s1 = "intention".chars();
    let s2 = "execution".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 5); // Expected distance for "intention" to "execution"
}

#[test]
fn test_damerau_levenshtein_edge_case_empty() {
    let s1 = "".chars();
    let s2 = "abc".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3); // Expected distance for "" to "abc"
}

#[test]
fn test_damerau_levenshtein_case_replacement() {
    let s1 = "abcde".chars();
    let s2 = "abfde".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1); // Expected distance for "abcde" to "abfde"
}

