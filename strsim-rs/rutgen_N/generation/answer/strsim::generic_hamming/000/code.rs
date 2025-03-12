// Answer 0

#[test]
fn test_generic_hamming_equal_sequences() {
    let seq1 = vec![1, 2, 3, 4, 5];
    let seq2 = vec![1, 2, 3, 4, 5];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_different_sequences() {
    let seq1 = vec![1, 2, 3, 4, 5];
    let seq2 = vec![1, 2, 0, 4, 5];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_generic_hamming_different_lengths() {
    let seq1 = vec![1, 2, 3];
    let seq2 = vec![1, 2, 3, 4];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_empty_sequences() {
    let seq1: Vec<i32> = vec![];
    let seq2: Vec<i32> = vec![];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_single_element_different() {
    let seq1 = vec![1];
    let seq2 = vec![2];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_generic_hamming_single_element_same() {
    let seq1 = vec![1];
    let seq2 = vec![1];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(0));
}

