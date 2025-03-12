// Answer 0

#[test]
fn test_end_with_valid_map_and_content() {
    struct MockMap {
        value: Option<Content>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, value: &Content) -> Result<(), Self::Error> {
            self.value = Some(value.clone());
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let fields = vec![
        ("field1", Content::U32(42)),
        ("field2", Content::String("example".to_string())),
    ];

    let map = MockMap { value: None };
    let serializer = SerializeStructVariantAsMapValue {
        map,
        name: "variant_name",
        fields,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_with_empty_fields() {
    struct MockMap {
        value: Option<Content>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, value: &Content) -> Result<(), Self::Error> {
            self.value = Some(value.clone());
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let fields: Vec<(&'static str, Content)> = vec![];

    let map = MockMap { value: None };
    let serializer = SerializeStructVariantAsMapValue {
        map,
        name: "empty_variant",
        fields,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_with_invalid_content() {
    struct MockMap {
        value: Option<Content>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let fields = vec![
        ("field1", Content::F64(3.14)),
    ];

    let map = MockMap { value: None };
    let serializer = SerializeStructVariantAsMapValue {
        map,
        name: "invalid_content",
        fields,
    };

    let _result = serializer.end();
}

#[test]
#[should_panic]
fn test_end_with_invalid_map_end() {
    struct MockMap;

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            panic!("Mock panic on end");
        }
    }

    let fields = vec![
        ("field1", Content::Char('a')),
    ];

    let map = MockMap;
    let serializer = SerializeStructVariantAsMapValue {
        map,
        name: "panic_test",
        fields,
    };

    let _ = serializer.end();
}

