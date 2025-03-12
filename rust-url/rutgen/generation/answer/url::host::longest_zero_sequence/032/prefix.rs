// Answer 0

#[test]
fn test_longest_zero_sequence_with_zeroes_followed_by_nonzero() {
    let pieces: [u16; 8] = [0, 0, 1, 0, 0, 0, 0, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_with_long_zeroes() {
    let pieces: [u16; 8] = [0, 0, 0, 0, 1, 1, 1, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_with_short_zeroes() {
    let pieces: [u16; 8] = [0, 0, 1, 1, 1, 1, 1, 1];
    longest_zero_sequence(&pieces);
}

