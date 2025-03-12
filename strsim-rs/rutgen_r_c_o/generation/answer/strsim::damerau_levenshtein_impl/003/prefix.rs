// Answer 0

#[test]
fn test_damerau_levenshtein_impl_case_1() {
    let s1 = "abc".chars();
    let s2 = "abx".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_impl_case_2() {
    let s1 = "hello".chars();
    let s2 = "hxllo".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_impl_case_3() {
    let s1 = "kitten".chars();
    let s2 = "sitten".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_impl_case_4() {
    let s1 = "flaw".chars();
    let s2 = "lawn".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

