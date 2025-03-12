// Answer 0

#[test]
fn test_backslash_x_byte_valid_a_to_f_then_digits() {
    let input = vec![(0, b'A'), (1, b'3'), (2, b'4')].into_iter();
    let result = backslash_x_byte(&mut input);
}

#[test]
fn test_backslash_x_byte_valid_b_to_f_then_digits() {
    let input = vec![(0, b'B'), (1, b'5'), (2, b'6')].into_iter();
    let result = backslash_x_byte(&mut input);
}

#[test]
fn test_backslash_x_byte_valid_c_to_f_then_digits() {
    let input = vec![(0, b'C'), (1, b'7'), (2, b'8')].into_iter();
    let result = backslash_x_byte(&mut input);
}

#[test]
fn test_backslash_x_byte_valid_d_to_f_then_digits() {
    let input = vec![(0, b'D'), (1, b'9'), (2, b'0')].into_iter();
    let result = backslash_x_byte(&mut input);
}

#[test]
fn test_backslash_x_byte_valid_e_to_f_then_digits() {
    let input = vec![(0, b'E'), (1, b'1'), (2, b'2')].into_iter();
    let result = backslash_x_byte(&mut input);
}

#[test]
fn test_backslash_x_byte_valid_f_then_digits() {
    let input = vec![(0, b'F'), (1, b'4'), (2, b'5')].into_iter();
    let result = backslash_x_byte(&mut input);
}

