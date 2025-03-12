// Answer 0

#[test]
fn test_serialize_u16_min() {
    let serializer = Serializer;
    let value: u16 = 0;
    let _ = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_mid() {
    let serializer = Serializer;
    let value: u16 = 32768;
    let _ = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_max() {
    let serializer = Serializer;
    let value: u16 = 65535;
    let _ = serializer.serialize_u16(value);
}

