// Answer 0

#[test]
fn test_begin_array_value_first_true_vec() {
    let mut writer = Vec::new();
    let mut formatter = StructFormatter;
    let result = formatter.begin_array_value(&mut writer, true);
}

#[test]
fn test_begin_array_value_first_true_string() {
    let mut writer = String::new();
    let mut formatter = StructFormatter;
    let result = formatter.begin_array_value(&mut writer, true);
}

#[test]
fn test_begin_array_value_first_true_bytes() {
    let mut writer = [0u8; 10];
    let mut formatter = StructFormatter;
    let result = formatter.begin_array_value(&mut writer, true);
}

struct StructFormatter;

impl Formatter for StructFormatter {}

