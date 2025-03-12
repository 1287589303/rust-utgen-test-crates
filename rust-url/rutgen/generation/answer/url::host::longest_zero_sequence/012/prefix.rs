// Answer 0

#[test]
fn test_longest_zero_sequence_case_one() {
    let pieces: [u16; 8] = [0, 0, 1, 0, 0, 1, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case_two() {
    let pieces: [u16; 8] = [1, 0, 0, 1, 0, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_edge_case() {
    let pieces: [u16; 8] = [0, 1, 1, 1, 1, 1, 1, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_no_zeros() {
    let pieces: [u16; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
    longest_zero_sequence(&pieces);
}

