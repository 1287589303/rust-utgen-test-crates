// Answer 0

#[test]
fn test_serialize_i64_min_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_i64(-9223372036854775808);
}

#[test]
fn test_serialize_i64_max_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_i64(9223372036854775807);
}

#[test]
fn test_serialize_i64_zero_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_negative_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_i64(-123456789);
}

#[test]
fn test_serialize_i64_positive_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_i64(123456789);
}

