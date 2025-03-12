// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u32(0);
}

#[test]
fn test_serialize_u32_one() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u32(1);
}

#[test]
fn test_serialize_u32_two() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u32(2);
}

#[test]
fn test_serialize_u32_boundary_min() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u32(u32::MIN);
}

#[test]
fn test_serialize_u32_boundary_max() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u32(u32::MAX);
}

#[test]
fn test_serialize_u32_mid_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u32(2_147_483_647);
}

