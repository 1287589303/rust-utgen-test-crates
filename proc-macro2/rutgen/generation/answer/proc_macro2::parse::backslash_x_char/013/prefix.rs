// Answer 0

#[test]
fn test_backslash_x_char_first_byte_valid_second_byte_a() {
    let input = vec![(0, '5'), (1, 'A')].into_iter();
    let result: Result<(), Reject> = backslash_x_char(&mut input);
}

#[test]
fn test_backslash_x_char_first_byte_valid_second_byte_b() {
    let input = vec![(0, '3'), (1, 'B')].into_iter();
    let result: Result<(), Reject> = backslash_x_char(&mut input);
}

#[test]
fn test_backslash_x_char_first_byte_valid_second_byte_c() {
    let input = vec![(0, '7'), (1, 'C')].into_iter();
    let result: Result<(), Reject> = backslash_x_char(&mut input);
}

#[test]
fn test_backslash_x_char_first_byte_valid_second_byte_d() {
    let input = vec![(0, '2'), (1, 'D')].into_iter();
    let result: Result<(), Reject> = backslash_x_char(&mut input);
}

#[test]
fn test_backslash_x_char_first_byte_valid_second_byte_e() {
    let input = vec![(0, '4'), (1, 'E')].into_iter();
    let result: Result<(), Reject> = backslash_x_char(&mut input);
}

#[test]
fn test_backslash_x_char_first_byte_valid_second_byte_f() {
    let input = vec![(0, '1'), (1, 'F')].into_iter();
    let result: Result<(), Reject> = backslash_x_char(&mut input);
}

