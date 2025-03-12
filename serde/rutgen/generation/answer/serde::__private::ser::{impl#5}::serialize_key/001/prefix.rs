// Answer 0

#[test]
fn test_serialize_key_valid_string() {
    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock = MockSerializeMap;
    let mut flat_map = FlatMapSerializeMap(&mut mock);
    flat_map.serialize_key(&"valid_string").unwrap();
}

#[test]
fn test_serialize_key_valid_number() {
    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock = MockSerializeMap;
    let mut flat_map = FlatMapSerializeMap(&mut mock);
    flat_map.serialize_key(&42).unwrap();
}

#[test]
fn test_serialize_key_invalid_type() {
    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error {})
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock = MockSerializeMap;
    let mut flat_map = FlatMapSerializeMap(&mut mock);
    let result = flat_map.serialize_key(&std::ptr::null());
    assert!(result.is_err());
}

