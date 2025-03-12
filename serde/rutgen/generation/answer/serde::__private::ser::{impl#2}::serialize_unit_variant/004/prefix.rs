// Answer 0

#[test]
fn test_serialize_unit_variant_success() {
    struct MockSerializer;

    impl SerializeMap for MockSerializer {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    struct TaggedSerializerTest {
        tag: &'static str,
        variant_name: &'static str,
        delegate: MockSerializer,
    }

    impl TaggedSerializerTest {
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            inner_variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            let mut map = self.delegate.serialize_map(Some(2)).unwrap();
            map.serialize_entry(self.tag, self.variant_name).unwrap();
            map.serialize_entry(inner_variant, &()).unwrap();
            map.end()
        }
    }

    let serializer = TaggedSerializerTest {
        tag: "type_tag",
        variant_name: "variant_name",
        delegate: MockSerializer,
    };

    let _result = serializer.serialize_unit_variant("test_type", 0, "inner_variant");
}

#[test]
fn test_serialize_unit_variant_failure_on_tag_entry() {
    struct FailingMockSerializer;

    impl SerializeMap for FailingMockSerializer {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    struct TaggedSerializerTest {
        tag: &'static str,
        variant_name: &'static str,
        delegate: FailingMockSerializer,
    }

    impl TaggedSerializerTest {
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            inner_variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            let mut map = self.delegate.serialize_map(Some(2)).unwrap();
            map.serialize_entry(self.tag, self.variant_name).unwrap();
            map.serialize_entry(inner_variant, &()).unwrap();
            map.end()
        }
    }

    let serializer = TaggedSerializerTest {
        tag: "type_tag",
        variant_name: "variant_name",
        delegate: FailingMockSerializer,
    };

    let _result = serializer.serialize_unit_variant("test_type", 0, "inner_variant");
}

