// Answer 0

#[test]
fn test_serialize_char_null() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_char('\u{0000}').unwrap();
}

#[test]
fn test_serialize_char_uppercase_a() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_char('\u{0041}').unwrap();
}

#[test]
fn test_serialize_char_del() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_char('\u{007F}').unwrap();
}

#[test]
fn test_serialize_char_surrogate_pair_start() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    // Surrogate pairs are invalid for character serialization
    let result = map_key_serializer.serialize_char('\u{D800}');
    assert!(result.is_err());
}

#[test]
fn test_serialize_char_surrogate_pair_end() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    // Surrogate pairs are invalid for character serialization
    let result = map_key_serializer.serialize_char('\u{DFFF}');
    assert!(result.is_err());
}

#[test]
fn test_serialize_char_replacement() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_char('\u{FFFD}').unwrap();
}

#[test]
fn test_serialize_char_maximum() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_char('\u{10FFFF}').unwrap();
}

