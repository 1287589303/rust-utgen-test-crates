// Answer 0

#[test]
fn test_write_literal_byte_ascii_control() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };
    let input = 0; // ASCII control character
    writer.write_literal_byte(input).unwrap();
}

#[test]
fn test_write_literal_byte_ascii_whitespace() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };
    let input = 32; // ASCII whitespace character
    writer.write_literal_byte(input).unwrap();
}

#[test]
fn test_write_literal_byte_above_127() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };
    let input = 128; // Just above the ASCII range
    writer.write_literal_byte(input).unwrap();
}

#[test]
fn test_write_literal_byte_high_value() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };
    let input = 255; // Maximum value for u8
    writer.write_literal_byte(input).unwrap();
}

