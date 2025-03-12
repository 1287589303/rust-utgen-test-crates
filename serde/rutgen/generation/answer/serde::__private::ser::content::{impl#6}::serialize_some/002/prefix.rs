// Answer 0

#[test]
fn test_serialize_some_string() {
    struct StringSerializable;

    impl Serialize for StringSerializable {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            serializer.serialize_str("test string")
        }
    }

    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_some(&StringSerializable);
}

#[test]
fn test_serialize_some_bool() {
    struct BoolSerializable;

    impl Serialize for BoolSerializable {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            serializer.serialize_bool(true)
        }
    }

    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_some(&BoolSerializable);
}

#[test]
fn test_serialize_some_u8() {
    struct U8Serializable;

    impl Serialize for U8Serializable {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            serializer.serialize_u8(42)
        }
    }

    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_some(&U8Serializable);
}

#[test]
fn test_serialize_some_seq() {
    struct VecSerializable;

    impl Serialize for VecSerializable {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            let mut seq = serializer.serialize_seq(Some(3))?;
            seq.serialize_element(&1)?;
            seq.serialize_element(&2)?;
            seq.serialize_element(&3)?;
            seq.end()
        }
    }

    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_some(&VecSerializable);
}

