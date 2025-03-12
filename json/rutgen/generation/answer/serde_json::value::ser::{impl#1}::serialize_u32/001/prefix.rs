// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(0);
}

#[test]
fn test_serialize_u32_one() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(1);
}

#[test]
fn test_serialize_u32_max() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(4_294_967_295);
}

#[test]
fn test_serialize_u32_mid() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(2_147_483_647);
}

