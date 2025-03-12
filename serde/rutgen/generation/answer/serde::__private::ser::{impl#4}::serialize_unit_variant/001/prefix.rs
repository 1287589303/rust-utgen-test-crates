// Answer 0

#[test]
fn test_serialize_unit_variant_valid() {
    struct MockMap {
        serialized: Vec<(String, ())>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key(&mut self, key: &str) -> Result<(), Self::Error> {
            self.serialized.push((key.to_string(), ()));
            Ok(())
        }

        fn serialize_entry(&mut self, _key: &str, _: &()) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { serialized: Vec::new() };
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_unit_variant("TestEnum", 0, "VariantA");
}

#[test]
fn test_serialize_unit_variant_non_empty_variant() {
    struct MockMap {
        serialized: Vec<(String, ())>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = ();

        fn serialize_key(&mut self, key: &str) -> Result<(), Self::Error> {
            self.serialized.push((key.to_string(), ()));
            Ok(())
        }

        fn serialize_entry(&mut self, _key: &str, _: &()) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { serialized: Vec::new() };
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_unit_variant("AnotherEnum", 1, "VariantB");
}

#[test]
#[should_panic]
fn test_serialize_unit_variant_invalid_index() {
    struct MockMap {
        serialized: Vec<(String, ())>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = ();

        fn serialize_key(&mut self, key: &str) -> Result<(), Self::Error> {
            self.serialized.push((key.to_string(), ()));
            Ok(())
        }

        fn serialize_entry(&mut self, _key: &str, _: &()) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { serialized: Vec::new() };
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_unit_variant("InvalidEnum", u32::MAX, "VariantC");
}

