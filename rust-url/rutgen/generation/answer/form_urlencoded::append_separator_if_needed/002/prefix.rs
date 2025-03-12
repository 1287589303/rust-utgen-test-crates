// Answer 0

#[test]
fn test_append_separator_no_change_when_len_eq_start_position() {
    let mut string = String::from("test");
    let start_position = string.len(); // string.len() == start_position
    append_separator_if_needed(&mut string, start_position);
}

#[test]
fn test_append_separator_no_change_for_empty_string() {
    let mut string = String::new();
    let start_position = string.len(); // string.len() == start_position
    append_separator_if_needed(&mut string, start_position);
}

#[test]
fn test_append_separator_no_change_when_len_non_zero_equals_start_position() {
    let mut string = String::from("abc");
    let start_position = string.len(); // string.len() == start_position
    append_separator_if_needed(&mut string, start_position);
}

