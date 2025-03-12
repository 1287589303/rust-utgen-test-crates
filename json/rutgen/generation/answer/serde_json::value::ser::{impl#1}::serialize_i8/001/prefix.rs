// Answer 0

#[test]
fn test_serialize_i8_min() {
    let serializer = Serializer;
    let _result = serializer.serialize_i8(-128);
}

#[test]
fn test_serialize_i8_zero() {
    let serializer = Serializer;
    let _result = serializer.serialize_i8(0);
}

#[test]
fn test_serialize_i8_max() {
    let serializer = Serializer;
    let _result = serializer.serialize_i8(127);
}

