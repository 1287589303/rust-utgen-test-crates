// Answer 0

#[test]
fn test_match_len_valid_case_min_length() {
    let data: [u8; 13] = [1 << 0 | 1 << 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // encoded length = 1
    let repr = Repr(&data);
    repr.match_len();
}

#[test]
fn test_match_len_valid_case_middle_length() {
    let data: [u8; 13] = [1 << 0 | 1 << 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5]; // encoded length = 5
    let repr = Repr(&data);
    repr.match_len();
}

#[test]
fn test_match_len_valid_case_max_length() {
    let data: [u8; 13] = [1 << 0 | 1 << 1, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255]; // encoded length = usize::max
    let repr = Repr(&data);
    repr.match_len();
}

#[test]
fn test_match_len_valid_case_zero_length() {
    let data: [u8; 13] = [1 << 0 | 1 << 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // encoded length = 0
    let repr = Repr(&data);
    repr.match_len();
}

