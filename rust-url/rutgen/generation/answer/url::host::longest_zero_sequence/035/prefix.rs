// Answer 0

#[test]
fn test_longest_zero_sequence_no_zeroes() {
    let pieces: [u16; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_one_zero() {
    let pieces: [u16; 8] = [1, 0, 1, 1, 1, 1, 1, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_no_consecutive_zeros() {
    let pieces: [u16; 8] = [1, 1, 0, 0, 1, 0, 1, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_maximum_zeros() {
    let pieces: [u16; 8] = [1, 1, 1, 1, 1, 1, 0, 1];
    longest_zero_sequence(&pieces);
}

