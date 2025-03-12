// Answer 0

#[test]
fn test_read_u64_valid_input() {
    let input: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 1];
    let _result = read_u64(input);
}

#[test]
fn test_read_u64_valid_input_max_value() {
    let input: &[u8] = &[255, 255, 255, 255, 255, 255, 255, 255];
    let _result = read_u64(input);
}

#[test]
#[should_panic]
fn test_read_u64_invalid_length_too_short() {
    let input: &[u8] = &[0, 0, 0, 0, 0, 0, 0];
    let _result = read_u64(input);
}

#[test]
#[should_panic]
fn test_read_u64_invalid_length_too_long() {
    let input: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0, 1];
    let _result = read_u64(input);
}

#[test]
#[should_panic]
fn test_read_u64_invalid_length_empty() {
    let input: &[u8] = &[];
    let _result = read_u64(input);
}

