// Answer 0

#[test]
fn test_serialize_infinite_positive() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: f32 = f32::INFINITY;
    serializer.serialize_f32(value).unwrap();
}

#[test]
fn test_serialize_infinite_negative() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: f32 = f32::NEG_INFINITY;
    serializer.serialize_f32(value).unwrap();
}

#[test]
fn test_serialize_nan() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: f32 = f32::NAN;
    serializer.serialize_f32(value).unwrap();
}

