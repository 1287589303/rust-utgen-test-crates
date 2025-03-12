// Answer 0

#[test]
#[should_panic]
fn test_serialize_element_failure_non_serializable() {
    struct NonSerializable;

    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = NonSerializable;

    let _result = serialize_vec.serialize_element(&value);
}

#[test]
#[should_panic]
fn test_serialize_element_failure_invalid_serialization() {
    struct InvalidSerialize;

    impl Serialize for InvalidSerialize {
        fn serialize<S>(&self, _: S) -> Result<Self::Ok, Self::Error>
        where
            S: serde::ser::Serializer,
        {
            Err(Error)
        }
    }

    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = InvalidSerialize;

    let _result = serialize_vec.serialize_element(&value);
}

