// Answer 0

#[test]
fn test_write_f64_positive_normal() {
    struct TestFormatter;

    let value = 1234.5678;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_negative_normal() {
    struct TestFormatter;

    let value = -1234.5678;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_zero() {
    struct TestFormatter;

    let value = 0.0;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_negative_zero() {
    struct TestFormatter;

    let value = -0.0;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_min() {
    struct TestFormatter;

    let value = f64::MIN;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_max() {
    struct TestFormatter;

    let value = f64::MAX;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_epsilon() {
    struct TestFormatter;

    let value = f64::EPSILON;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_infinity() {
    struct TestFormatter;

    let value = f64::INFINITY;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

#[test]
fn test_write_f64_negative_infinity() {
    struct TestFormatter;

    let value = f64::NEG_INFINITY;
    let mut writer = Vec::new();
    TestFormatter.write_f64(&mut writer, value).unwrap();
}

