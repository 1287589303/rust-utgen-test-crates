// Answer 0

#[test]
fn test_generic_hamming_different_lengths_error() {
    let sequence_a = vec![1, 2, 3];
    let sequence_b = vec![1, 2];

    let result = generic_hamming(sequence_a, sequence_b);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_different_positions_count() {
    let sequence_a = vec![1, 2, 3];
    let sequence_b = vec![1, 3, 2];

    let result = generic_hamming(sequence_a, sequence_b);
    assert_eq!(result, Ok(2));
}

