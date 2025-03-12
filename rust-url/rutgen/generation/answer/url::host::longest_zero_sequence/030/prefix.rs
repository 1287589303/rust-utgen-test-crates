// Answer 0

#[test]
fn test_longest_zero_sequence_case1() {
    let pieces: [u16; 8] = [0, 0, 1, 0, 0, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case2() {
    let pieces: [u16; 8] = [0, 0, 0, 1, 0, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case3() {
    let pieces: [u16; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case4() {
    let pieces: [u16; 8] = [0, 1, 0, 1, 0, 1, 0, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case5() {
    let pieces: [u16; 8] = [0, 0, 0, 0, 1, 1, 1, 1];
    longest_zero_sequence(&pieces);
}

#[test]
fn test_longest_zero_sequence_case6() {
    let pieces: [u16; 8] = [0, 0, 1, 1, 0, 0, 0, 0];
    longest_zero_sequence(&pieces);
}

