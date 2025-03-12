// Answer 0

#[test]
fn test_serialize_u64_min_value() {
    let value: u64 = 0;
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    map_key_serializer.serialize_u64(value).unwrap();
}

#[test]
fn test_serialize_u64_mid_value() {
    let value: u64 = 1_000_000;
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    map_key_serializer.serialize_u64(value).unwrap();
}

#[test]
fn test_serialize_u64_max_value() {
    let value: u64 = u64::MAX;
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    map_key_serializer.serialize_u64(value).unwrap();
}

