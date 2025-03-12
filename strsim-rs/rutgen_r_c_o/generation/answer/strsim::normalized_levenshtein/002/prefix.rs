// Answer 0

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_length_1() {
    let a = "";
    let b = "a";
    normalized_levenshtein(a, b);
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_length_10() {
    let a = "";
    let b = "abcdefghij";
    normalized_levenshtein(a, b);
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_length_50() {
    let a = "";
    let b = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";
    normalized_levenshtein(a, b);
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_length_100() {
    let a = "";
    let b = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";
    normalized_levenshtein(a, b);
}

