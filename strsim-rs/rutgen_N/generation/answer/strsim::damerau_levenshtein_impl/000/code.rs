// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 0, s2, 0), 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    let s1 = "abc".chars();
    let s2 = "".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 3, s2, 0), 3);

    let s1 = "".chars();
    let s2 = "abc".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 0, s2, 3), 3);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1 = "hello".chars();
    let s2 = "hello".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 5, s2, 5), 0);
}

#[test]
fn test_damerau_levenshtein_insertions() {
    let s1 = "abc".chars();
    let s2 = "abcd".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 3, s2, 4), 1);
}

#[test]
fn test_damerau_levenshtein_deletions() {
    let s1 = "abcd".chars();
    let s2 = "abc".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 4, s2, 3), 1);
}

#[test]
fn test_damerau_levenshtein_replacements() {
    let s1 = "abc".chars();
    let s2 = "axc".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 3, s2, 3), 2);
}

#[test]
fn test_damerau_levenshtein_transpositions() {
    let s1 = "ab".chars();
    let s2 = "ba".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 2, s2, 2), 1);
}

#[test]
fn test_damerau_levenshtein_longer_strings() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 6, s2, 7), 3);
}

