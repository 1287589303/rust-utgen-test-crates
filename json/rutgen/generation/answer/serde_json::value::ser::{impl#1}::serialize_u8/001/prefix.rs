// Answer 0

#[test]
fn test_serialize_u8_0() {
    let serializer = Serializer;
    let _result = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_255() {
    let serializer = Serializer;
    let _result = serializer.serialize_u8(255);
}

#[test]
fn test_serialize_u8_128() {
    let serializer = Serializer;
    let _result = serializer.serialize_u8(128);
}

#[test]
fn test_serialize_u8_1() {
    let serializer = Serializer;
    let _result = serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_254() {
    let serializer = Serializer;
    let _result = serializer.serialize_u8(254);
}

