// Answer 0

#[test]
fn test_write_i16_min_value() {
    struct TestFormatter;
    let mut writer = Vec::new();
    let value: i16 = -32_768;
    let mut formatter = TestFormatter;
    let _ = formatter.write_i16(&mut writer, value);
}

#[test]
fn test_write_i16_negative_value() {
    struct TestFormatter;
    let mut writer = Vec::new();
    let value: i16 = -1;
    let mut formatter = TestFormatter;
    let _ = formatter.write_i16(&mut writer, value);
}

#[test]
fn test_write_i16_zero_value() {
    struct TestFormatter;
    let mut writer = Vec::new();
    let value: i16 = 0;
    let mut formatter = TestFormatter;
    let _ = formatter.write_i16(&mut writer, value);
}

#[test]
fn test_write_i16_positive_value() {
    struct TestFormatter;
    let mut writer = Vec::new();
    let value: i16 = 1;
    let mut formatter = TestFormatter;
    let _ = formatter.write_i16(&mut writer, value);
}

#[test]
fn test_write_i16_max_value() {
    struct TestFormatter;
    let mut writer = Vec::new();
    let value: i16 = 32_767;
    let mut formatter = TestFormatter;
    let _ = formatter.write_i16(&mut writer, value);
}

