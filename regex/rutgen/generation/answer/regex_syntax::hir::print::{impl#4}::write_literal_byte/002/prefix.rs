// Answer 0

#[test]
fn test_write_literal_byte_space() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    writer.write_literal_byte(0x20).unwrap();
}

#[test]
fn test_write_literal_byte_tab() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    writer.write_literal_byte(0x09).unwrap();
}

#[test]
fn test_write_literal_byte_line_feed() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    writer.write_literal_byte(0x0A).unwrap();
}

#[test]
fn test_write_literal_byte_form_feed() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    writer.write_literal_byte(0x0C).unwrap();
}

#[test]
fn test_write_literal_byte_carriage_return() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    writer.write_literal_byte(0x0D).unwrap();
}

