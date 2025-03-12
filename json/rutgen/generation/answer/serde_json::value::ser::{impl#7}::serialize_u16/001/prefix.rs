// Answer 0

#[test]
fn test_serialize_u16_zero() {
    let serializer = MapKeySerializer;
    let value: u16 = 0;
    let _result = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_min() {
    let serializer = MapKeySerializer;
    let value: u16 = 1;
    let _result = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_middle() {
    let serializer = MapKeySerializer;
    let value: u16 = 32768;
    let _result = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_max() {
    let serializer = MapKeySerializer;
    let value: u16 = 65535;
    let _result = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_large() {
    let serializer = MapKeySerializer;
    let value: u16 = 10000;
    let _result = serializer.serialize_u16(value);
}

