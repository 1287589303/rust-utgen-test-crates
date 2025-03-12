// Answer 0

#[test]
fn test_serialize_u128_zero() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    key_serializer.serialize_u128(0).expect("Serialization should succeed");
}

#[test]
fn test_serialize_u128_mid_range() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    key_serializer.serialize_u128(170141183460469231731687303715884105727).expect("Serialization should succeed");
}

#[test]
fn test_serialize_u128_max() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    key_serializer.serialize_u128(u128::MAX).expect("Serialization should succeed");
}

