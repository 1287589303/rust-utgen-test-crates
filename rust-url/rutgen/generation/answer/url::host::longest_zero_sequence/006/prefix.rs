// Answer 0

#[test]
fn test_longest_zero_sequence_case_1() {
    let pieces: [u16; 8] = [0, 0, 0, 1, 0, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case_2() {
    let pieces: [u16; 8] = [1, 0, 0, 0, 1, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case_3() {
    let pieces: [u16; 8] = [1, 1, 1, 1, 0, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case_4() {
    let pieces: [u16; 8] = [0, 0, 1, 1, 0, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case_5() {
    let pieces: [u16; 8] = [1, 1, 0, 0, 0, 0, 1, 1];
    longest_zero_sequence(&pieces);
}

