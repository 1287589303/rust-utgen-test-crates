// Answer 0

#[test]
fn test_flat_map_serialize_map_end() {
    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockSerializeMap;
    let flat_map = FlatMapSerializeMap(&mut mock_map);
    let _result = flat_map.end();
}

