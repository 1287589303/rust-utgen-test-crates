// Answer 0

#[test]
fn test_generic_hamming_equal_sequences() {
    let seq1 = vec![1, 2, 3, 4];
    let seq2 = vec![1, 2, 3, 4];

    let result = generic_hamming(&seq1, &seq2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_empty_sequences() {
    let seq1: Vec<u8> = Vec::new();
    let seq2: Vec<u8> = Vec::new();

    let result = generic_hamming(&seq1, &seq2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_equal_sequences_with_chars() {
    let seq1 = vec!['a', 'b', 'c'];
    let seq2 = vec!['a', 'b', 'c'];

    let result = generic_hamming(&seq1, &seq2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_mixed_equal_sequences() {
    let seq1 = vec![1.0, 2.0, 3.0];
    let seq2 = vec![1.0, 2.0, 3.0];

    let result = generic_hamming(&seq1, &seq2);
    assert_eq!(result, Ok(0));
}

