// Answer 0

#[test]
fn test_serialize_f64_negative_normal() {
    let mut writer = Vec::new();
    let mut formatter = MockFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };

    let value: f64 = -123.456;
    serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_zero() {
    let mut writer = Vec::new();
    let mut formatter = MockFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };

    let value: f64 = 0.0;
    serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_positive_normal() {
    let mut writer = Vec::new();
    let mut formatter = MockFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };

    let value: f64 = 123.456;
    serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_large_positive() {
    let mut writer = Vec::new();
    let mut formatter = MockFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };

    let value: f64 = 1.7976931348623157E+308; // MAX value
    serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_large_negative() {
    let mut writer = Vec::new();
    let mut formatter = MockFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };

    let value: f64 = -1.7976931348623157E+308; // MIN value
    serializer.serialize_f64(value);
}

