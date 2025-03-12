// Answer 0

#[test]
fn test_end_with_valid_flatmap_serializestruct() {
    struct DummyMap;

    impl SerializeMap for DummyMap {
        type Error = ();

        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = DummyMap;
    let mut struct_serializer = FlatMapSerializeStruct(&mut map);
    let result = struct_serializer.end();
}

#[test]
fn test_end_with_multiple_serializations() {
    struct DummyMap;

    impl SerializeMap for DummyMap {
        type Error = ();

        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = DummyMap;
    let mut struct_serializer = FlatMapSerializeStruct(&mut map);
    struct_serializer.serialize_field("key1", &"value1").unwrap();
    struct_serializer.serialize_field("key2", &"value2").unwrap();
    let result = struct_serializer.end();
}

#[test]
fn test_end_with_no_serializations() {
    struct DummyMap;

    impl SerializeMap for DummyMap {
        type Error = ();

        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = DummyMap;
    let mut struct_serializer = FlatMapSerializeStruct(&mut map);
    let result = struct_serializer.end();
}

