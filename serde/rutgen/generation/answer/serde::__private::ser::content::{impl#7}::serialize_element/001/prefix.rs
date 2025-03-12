// Answer 0

#[test]
fn test_serialize_element_bool_error() {
    struct ErrSerializer;

    impl Serialize for ErrSerializer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(/* insert appropriate error here */)
        }
    }

    let mut seq = SerializeSeq::<()>::new();
    let error = seq.serialize_element(&ErrSerializer);
}

#[test]
fn test_serialize_element_u8_error() {
    struct ErrSerializer;

    impl Serialize for ErrSerializer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(/* insert appropriate error here */)
        }
    }

    let mut seq = SerializeSeq::<()>::new();
    let error = seq.serialize_element(&ErrSerializer);
}

#[test]
fn test_serialize_element_string_error() {
    struct ErrSerializer;

    impl Serialize for ErrSerializer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(/* insert appropriate error here */)
        }
    }

    let mut seq = SerializeSeq::<()>::new();
    let error = seq.serialize_element(&ErrSerializer);
}

