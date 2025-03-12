// Answer 0

#[test]
fn test_serialize_f64_nan() {
    let value: f64 = std::f64::NAN;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let serializer = Serializer {
        writer,
        formatter,
    };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _result = map_key_serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_infinity() {
    let value: f64 = std::f64::INFINITY;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let serializer = Serializer {
        writer,
        formatter,
    };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _result = map_key_serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_negative_infinity() {
    let value: f64 = std::f64::NEG_INFINITY;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let serializer = Serializer {
        writer,
        formatter,
    };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _result = map_key_serializer.serialize_f64(value);
}

