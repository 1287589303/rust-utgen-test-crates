// Answer 0

#[test]
fn test_serialize_map_none() {
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer: Vec::new(),
            formatter: CompactFormatter,
        },
    };
    let result = serializer.serialize_map(None);
}

#[test]
fn test_serialize_map_some_zero() {
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer: Vec::new(),
            formatter: CompactFormatter,
        },
    };
    let result = serializer.serialize_map(Some(0));
}

#[test]
fn test_serialize_map_some_positive() {
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer: Vec::new(),
            formatter: CompactFormatter,
        },
    };
    let result = serializer.serialize_map(Some(5));
}

