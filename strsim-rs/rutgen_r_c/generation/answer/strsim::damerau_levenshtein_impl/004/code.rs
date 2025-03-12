// Answer 0

fn test_damerau_levenshtein_impl_case1() {
    let s1 = "abc".chars();
    let s2 = "acb".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 2);
}

fn test_damerau_levenshtein_impl_case2() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

fn test_damerau_levenshtein_impl_case3() {
    let s1 = "flaw".chars();
    let s2 = "lawn".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 2);
}

fn test_damerau_levenshtein_impl_case4() {
    let s1 = "abcdef".chars();
    let s2 = "azced".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

