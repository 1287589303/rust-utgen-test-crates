// Answer 0

#[test]
fn test_serialize_newtype_variant_map_err() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = TestSerializeMap;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(TestSerializeMap { entries: vec![] })
        }

        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            inner_variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_newtype_variant<T>(
            self,
            _: &'static str,
            _: u32,
            inner_variant: &'static str,
            inner_value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let mut map = tri!(self.serialize_map(Some(2)));
            tri!(map.serialize_entry("tag", "variant"));
            tri!(map.serialize_entry(inner_variant, inner_value));
            map.end()
        }
    }

    struct TestSerializeMap {
        entries: Vec<(Content, Content)>,
    }

    impl SerializeMap for TestSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            Err(Error)
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let result: Result<(), Error> = serializer.serialize_newtype_variant("test", 0, "inner", &42);
    let _ = result; // Consume the result to avoid warnings
}

#[test]
fn test_serialize_newtype_variant_non_serializable() {
    struct TestNonSerializable;

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = TestSerializeMap;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(TestSerializeMap { entries: vec![] })
        }

        fn serialize_newtype_variant<T>(
            self,
            _: &'static str,
            _: u32,
            inner_variant: &'static str,
            inner_value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let mut map = tri!(self.serialize_map(Some(2)));
            tri!(map.serialize_entry("tag", "variant"));
            tri!(map.serialize_entry(inner_variant, inner_value));
            map.end()
        }
    }

    struct TestSerializeMap {
        entries: Vec<(Content, Content)>,
    }

    impl SerializeMap for TestSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let result: Result<(), Error> = serializer.serialize_newtype_variant("test", 0, "inner", &TestNonSerializable);
    let _ = result; // Consume the result to avoid warnings
}

