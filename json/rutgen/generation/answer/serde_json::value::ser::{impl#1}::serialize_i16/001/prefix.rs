// Answer 0

#[test]
fn test_serialize_i16_min() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(-32_768);
}

#[test]
fn test_serialize_i16_negative_one() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(-1);
}

#[test]
fn test_serialize_i16_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_positive_one() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(1);
}

#[test]
fn test_serialize_i16_max() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(32_767);
}

