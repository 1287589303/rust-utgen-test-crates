// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<K>(&mut self, _: K) -> Result<(), Self::Error>
        where
            K: Serialize,
        {
            Ok(())
        }

        fn serialize_entry<K, V>(&mut self, _: K, _: V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_false() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<K>(&mut self, _: K) -> Result<(), Self::Error>
        where
            K: Serialize,
        {
            Ok(())
        }

        fn serialize_entry<K, V>(&mut self, _: K, _: V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_bool(false);
}

