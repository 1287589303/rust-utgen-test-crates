// Answer 0

#[test]
fn test_normalized_levenshtein_empty_strings() {
    let result = normalized_levenshtein("", "");
}

#[test]
fn test_normalized_levenshtein_first_empty() {
    let result = normalized_levenshtein("", "second");
}

#[test]
fn test_normalized_levenshtein_second_empty() {
    let result = normalized_levenshtein("first", "");
}

#[test]
fn test_normalized_levenshtein_same_strings() {
    let result = normalized_levenshtein("string", "string");
}

#[test]
fn test_normalized_levenshtein_different_strings() {
    let result = normalized_levenshtein("kitten", "sitting");
}

