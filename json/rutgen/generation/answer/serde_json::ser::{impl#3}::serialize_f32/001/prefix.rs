// Answer 0

#[test]
fn test_serialize_f32_finite_positive() {
    let writer = Vec::new();
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };
    let value: f32 = 1.0; // A finite positive number
    serializer.serialize_f32(value).unwrap();
}

#[test]
fn test_serialize_f32_finite_negative() {
    let writer = Vec::new();
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };
    let value: f32 = -1.0; // A finite negative number
    serializer.serialize_f32(value).unwrap();
}

#[test]
fn test_serialize_f32_finite_max() {
    let writer = Vec::new();
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };
    let value: f32 = 3.4028235e38; // Maximum finite f32 number
    serializer.serialize_f32(value).unwrap();
}

#[test]
fn test_serialize_f32_finite_min() {
    let writer = Vec::new();
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };
    let value: f32 = -3.4028235e38; // Minimum finite f32 number
    serializer.serialize_f32(value).unwrap();
}

#[test]
fn test_serialize_f32_finite_mid() {
    let writer = Vec::new();
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };
    let value: f32 = 0.5; // A finite middle number
    serializer.serialize_f32(value).unwrap();
}

