// Answer 0

#[test]
fn test_serialize_u16_min() {
    let value: u16 = 0;
    let mut map: Vec<(&str,())> = Vec::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_mid() {
    let value: u16 = 1;
    let mut map: Vec<(&str,())> = Vec::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_half() {
    let value: u16 = 32768;
    let mut map: Vec<(&str,())> = Vec::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_second_last() {
    let value: u16 = 65534;
    let mut map: Vec<(&str,())> = Vec::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_max() {
    let value: u16 = 65535;
    let mut map: Vec<(&str,())> = Vec::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u16(value);
}

