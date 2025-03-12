// Answer 0

#[test]
fn test_serialize_key_with_string_key() {
    struct StringKey(String);
    impl Serialize for StringKey {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0)
        }
    }

    let mut map = SerializeMap::Map {
        map: Map { map: Default::default() },
        next_key: None,
    };
    let key = StringKey("test_key".to_string());
    let _ = map.serialize_key(&key);
}

#[test]
fn test_serialize_key_with_int_key() {
    struct IntKey(i32);
    impl Serialize for IntKey {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_i32(self.0)
        }
    }

    let mut map = SerializeMap::Map {
        map: Map { map: Default::default() },
        next_key: None,
    };
    let key = IntKey(42);
    let _ = map.serialize_key(&key);
}

#[test]
fn test_serialize_key_with_bool_key() {
    struct BoolKey(bool);
    impl Serialize for BoolKey {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bool(self.0)
        }
    }

    let mut map = SerializeMap::Map {
        map: Map { map: Default::default() },
        next_key: None,
    };
    let key = BoolKey(true);
    let _ = map.serialize_key(&key);
}

