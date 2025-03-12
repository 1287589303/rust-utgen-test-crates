// Answer 0

#[test]
fn test_damerau_levenshtein_impl_no_characters() {
    let s1 = "".chars();
    let len1 = 0;
    let s2 = "".chars();
    let len2 = 0;

    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_impl_one_empty_string() {
    let s1 = "abc".chars();
    let len1 = 3;
    let s2 = "".chars();
    let len2 = 0;

    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_impl_single_character_difference() {
    let s1 = "a".chars();
    let len1 = 1;
    let s2 = "b".chars();
    let len2 = 1;

    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_impl_transpose_characters() {
    let s1 = "ab".chars();
    let len1 = 2;
    let s2 = "ba".chars();
    let len2 = 2;

    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_impl_multiple_characters_insertions() {
    let s1 = "abc".chars();
    let len1 = 3;
    let s2 = "abcdef".chars();
    let len2 = 6;

    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_impl_multiple_characters_deletions() {
    let s1 = "abcdef".chars();
    let len1 = 6;
    let s2 = "abc".chars();
    let len2 = 3;

    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_impl_multiple_character_replacements() {
    let s1 = "kitten".chars();
    let len1 = 6;
    let s2 = "sitting".chars();
    let len2 = 7;

    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

