// Answer 0

fn test_damerau_levenshtein_impl_case_1() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3); // Expected edit distance for this case
}

fn test_damerau_levenshtein_impl_case_2() {
    let s1 = "flaw".chars();
    let s2 = "lawn".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 2); // Expected edit distance for this case
}

fn test_damerau_levenshtein_impl_case_3() {
    let s1 = "example".chars();
    let s2 = "samples".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3); // Expected edit distance for this case
}

fn test_damerau_levenshtein_impl_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 0); // Edit distance between two empty strings is 0
}

fn test_damerau_levenshtein_impl_case_with_different_length() {
    let s1 = "short".chars();
    let s2 = "longerstring".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 10); // Expected edit distance for this case
}

