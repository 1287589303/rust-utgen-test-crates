// Answer 0

#[test]
fn test_serialize_newtype_variant_valid() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = TestMap;
        type SerializeStruct = ();
        
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(TestMap::new())
        }

        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_newtype_variant<T>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    struct TestMap {
        entries: Vec<(&'static str, &'static str)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
    }

    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.entries.push((key.serialize_str(key), ""));
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.entries.last_mut().unwrap().1 = value.serialize_str(value);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let tag = "tag_name";
    let variant_name = "variant_name";
    let inner_variant = "inner_variant_name";
    let inner_value = "inner_value";

    let _ = serializer.serialize_newtype_variant("test_type", 0, inner_variant, &inner_value);
}

#[test]
fn test_serialize_newtype_variant_with_different_value() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = TestMap;
        type SerializeStruct = ();
        
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(TestMap::new())
        }

        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_newtype_variant<T>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    struct TestMap {
        entries: Vec<(&'static str, &'static str)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
    }

    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.entries.push((key.serialize_str(key), ""));
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.entries.last_mut().unwrap().1 = value.serialize_str(value);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let tag = "tag_name";
    let variant_name = "variant_name";
    let inner_variant = "another_inner_variant";
    let inner_value = "another_inner_value";

    let _ = serializer.serialize_newtype_variant("test_type", 1, inner_variant, &inner_value);
}

