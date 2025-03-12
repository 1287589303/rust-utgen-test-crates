// Answer 0

#[test]
fn test_is_char_boundary_valid_utf8_start_boundary() {
    let input = Input::new("☃");
    input.is_char_boundary(0);
}

#[test]
fn test_is_char_boundary_valid_utf8_inside_character() {
    let input = Input::new("☃");
    input.is_char_boundary(1);
}

#[test]
fn test_is_char_boundary_valid_utf8_end_boundary() {
    let input = Input::new("☃");
    input.is_char_boundary(3);
}

#[test]
fn test_is_char_boundary_valid_utf8_out_of_bounds() {
    let input = Input::new("☃");
    input.is_char_boundary(4);
}

#[test]
fn test_is_char_boundary_multiple_codepoints() {
    let input = Input::new("こんにちは");
    input.is_char_boundary(0);
    input.is_char_boundary(3);
    input.is_char_boundary(6);
    input.is_char_boundary(15);
}

#[test]
fn test_is_char_boundary_invalid_utf8() {
    let input = Input::new(&[0, 159, 146, 150]); // Invalid UTF-8 sequence
    input.is_char_boundary(0);
    input.is_char_boundary(1);
    input.is_char_boundary(2);
    input.is_char_boundary(3);
}

