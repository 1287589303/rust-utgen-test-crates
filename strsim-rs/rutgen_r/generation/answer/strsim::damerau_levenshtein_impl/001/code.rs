// Answer 0

#[test]
fn test_damerau_levenshtein_equal_strings() {
    let s1 = "test".chars();
    let s2 = "test".chars();
    let len1 = 4;
    let len2 = 4;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_one_insert() {
    let s1 = "test".chars();
    let s2 = "tests".chars();
    let len1 = 4;
    let len2 = 5;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_one_delete() {
    let s1 = "tests".chars();
    let s2 = "test".chars();
    let len1 = 5;
    let len2 = 4;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_one_substitute() {
    let s1 = "test".chars();
    let s2 = "best".chars();
    let len1 = 4;
    let len2 = 4;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_two_operations() {
    let s1 = "abc".chars();
    let s2 = "xyc".chars();
    let len1 = 3;
    let len2 = 3;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 2);
}

#[test]
fn test_damerau_levenshtein_three_operations() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let len1 = 6;
    let len2 = 7;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let len1 = 0;
    let len2 = 0;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_non_empty_and_empty() {
    let s1 = "nonempty".chars();
    let s2 = "".chars();
    let len1 = 9;
    let len2 = 0;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, len1); 
} 

#[test]
fn test_damerau_levenshtein_large_diff() {
    let s1 = "kitten".chars();
    let s2 = "sittingdifferently".chars();
    let len1 = 6;
    let len2 = 19;
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 15);
}

