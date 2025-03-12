// Answer 0

#[test]
fn test_damerau_levenshtein_basic_cases() {
    assert_eq!(damerau_levenshtein("ab", "bca"), 2);
    assert_eq!(damerau_levenshtein("abc", "abc"), 0);
    assert_eq!(damerau_levenshtein("abc", "ab"), 1);
    assert_eq!(damerau_levenshtein("ab", "abc"), 1);
}

#[test]
fn test_damerau_levenshtein_empty_strings() {
    assert_eq!(damerau_levenshtein("", ""), 0);
    assert_eq!(damerau_levenshtein("a", ""), 1);
    assert_eq!(damerau_levenshtein("", "a"), 1);
}

#[test]
fn test_damerau_levenshtein_special_characters() {
    assert_eq!(damerau_levenshtein("!@#", "#@!"), 2);
    assert_eq!(damerau_levenshtein("abc!@#", "abc#@!"), 2);
}

#[test]
fn test_damerau_levenshtein_with_same_characters() {
    assert_eq!(damerau_levenshtein("aaa", "aaaa"), 1);
    assert_eq!(damerau_levenshtein("aaaa", "aaa"), 1);
}

#[test]
fn test_damerau_levenshtein_large_input() {
    let a = "a".repeat(1000);
    let b = "a".repeat(999) + "b";
    assert_eq!(damerau_levenshtein(&a, &b), 1);
}

