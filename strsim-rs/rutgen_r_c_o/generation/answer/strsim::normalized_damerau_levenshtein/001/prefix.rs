// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_both_empty_strings() {
    let result = normalized_damerau_levenshtein("", "");
}

#[test]
fn test_normalized_damerau_levenshtein_first_empty_second_non_empty() {
    let result = normalized_damerau_levenshtein("", "flower");
}

#[test]
fn test_normalized_damerau_levenshtein_first_non_empty_second_empty() {
    let result = normalized_damerau_levenshtein("tree", "");
}

