// Answer 0

#[test]
fn test_generic_levenshtein_equal_sequences() {
    let seq1 = &[1, 2, 3];
    let seq2 = &[1, 2, 3];
    assert_eq!(generic_levenshtein(seq1, seq2), 0);
}

#[test]
fn test_generic_levenshtein_insertions() {
    let seq1 = &[1, 2, 3];
    let seq2 = &[1, 2, 3, 4, 5, 6];
    assert_eq!(generic_levenshtein(seq1, seq2), 3);
}

#[test]
fn test_generic_levenshtein_deletions() {
    let seq1 = &[1, 2, 3, 4, 5, 6];
    let seq2 = &[1, 2, 3];
    assert_eq!(generic_levenshtein(seq1, seq2), 3);
}

#[test]
fn test_generic_levenshtein_substitutions() {
    let seq1 = &[1, 2, 3];
    let seq2 = &[2, 3, 4];
    assert_eq!(generic_levenshtein(seq1, seq2), 2);
}

#[test]
fn test_generic_levenshtein_empty_first_sequence() {
    let seq1: &[i32] = &[];
    let seq2 = &[1, 2, 3];
    assert_eq!(generic_levenshtein(seq1, seq2), 3);
}

#[test]
fn test_generic_levenshtein_empty_second_sequence() {
    let seq1 = &[1, 2, 3];
    let seq2: &[i32] = &[];
    assert_eq!(generic_levenshtein(seq1, seq2), 3);
}

#[test]
fn test_generic_levenshtein_both_empty_sequences() {
    let seq1: &[i32] = &[];
    let seq2: &[i32] = &[];
    assert_eq!(generic_levenshtein(seq1, seq2), 0);
}

