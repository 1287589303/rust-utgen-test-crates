// Answer 0

#[test]
fn test_damerau_levenshtein_impl_basic() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let distance = damerau_levenshtein_impl(s1, 6, s2, 7);
    assert_eq!(distance, 3);
}

#[test]
fn test_damerau_levenshtein_impl_identical() {
    let s1 = "example".chars();
    let s2 = "example".chars();
    let distance = damerau_levenshtein_impl(s1, 7, s2, 7);
    assert_eq!(distance, 0);
}

#[test]
fn test_damerau_levenshtein_impl_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let distance = damerau_levenshtein_impl(s1, 0, s2, 0);
    assert_eq!(distance, 0);
}

#[test]
fn test_damerau_levenshtein_impl_first_empty() {
    let s1 = "nonempty".chars();
    let s2 = "".chars();
    let distance = damerau_levenshtein_impl(s1, 9, s2, 0);
    assert_eq!(distance, 9);
}

#[test]
fn test_damerau_levenshtein_impl_second_empty() {
    let s1 = "".chars();
    let s2 = "nonempty".chars();
    let distance = damerau_levenshtein_impl(s1, 0, s2, 9);
    assert_eq!(distance, 9);
}

#[test]
fn test_damerau_levenshtein_impl_transposition() {
    let s1 = "converse".chars();
    let s2 = "conserve".chars();
    let distance = damerau_levenshtein_impl(s1, 8, s2, 8);
    assert_eq!(distance, 1);
}

#[test]
fn test_damerau_levenshtein_impl_case_sensitive() {
    let s1 = "hello".chars();
    let s2 = "Hello".chars();
    let distance = damerau_levenshtein_impl(s1, 5, s2, 5);
    assert_eq!(distance, 1);
}

