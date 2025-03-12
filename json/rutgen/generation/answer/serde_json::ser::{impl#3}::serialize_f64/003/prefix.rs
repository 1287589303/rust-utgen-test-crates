// Answer 0

#[test]
fn test_serialize_f64_nan() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let value = f64::NAN;

    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_infinity() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let value = f64::INFINITY;

    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_negative_infinity() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let value = f64::NEG_INFINITY;

    let _ = serializer.serialize_f64(value);
}

