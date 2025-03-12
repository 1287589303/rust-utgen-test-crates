// Answer 0

#[test]
fn test_damerau_levenshtein_identical_strings() {
    assert_eq!(0, strsim::damerau_levenshtein("test", "test"));
}

#[test]
fn test_damerau_levenshtein_one_character_difference() {
    assert_eq!(1, strsim::damerau_levenshtein("a", "b"));
}

#[test]
fn test_damerau_levenshtein_two_characters_difference() {
    assert_eq!(2, strsim::damerau_levenshtein("ab", "cd"));
}

#[test]
fn test_damerau_levenshtein_insertions_and_deletions() {
    assert_eq!(3, strsim::damerau_levenshtein("kitten", "sitting"));
}

#[test]
fn test_damerau_levenshtein_duplicates() {
    assert_eq!(2, strsim::damerau_levenshtein("aabb", "ab"));
}

#[test]
fn test_damerau_levenshtein_empty_string() {
    assert_eq!(5, strsim::damerau_levenshtein("", "abcde"));
}

#[test]
fn test_damerau_levenshtein_large_difference() {
    assert_eq!(6, strsim::damerau_levenshtein("abcdef", "ghijkl"));
}

#[test]
fn test_damerau_levenshtein_substring() {
    assert_eq!(3, strsim::damerau_levenshtein("abc", "aabcdef"));
}

