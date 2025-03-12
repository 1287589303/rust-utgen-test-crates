// Answer 0

#[test]
fn test_end_with_valid_map_and_non_empty_fields() {
    struct MockSerializeMap {
        called: bool,
    }

    impl MockSerializeMap {
        fn new() -> Self {
            Self { called: false }
        }
    }

    impl ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            self.called = true;
            Ok(())
        }

        fn serialize_entry(&mut self, _: &Content, _: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockSerializeMap::new();
    let fields = vec![
        Content::Bool(true),
        Content::U8(42),
        Content::String("test".to_string()),
    ];
    
    let mut serializer = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };

    let _ = serializer.end();
    assert!(map.called);
}

#[test]
fn test_end_with_empty_map_and_non_empty_fields() {
    struct MockSerializeMap {
        called: bool,
    }

    impl MockSerializeMap {
        fn new() -> Self {
            Self { called: false }
        }
    }

    impl ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            self.called = true;
            Ok(())
        }

        fn serialize_entry(&mut self, _: &Content, _: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockSerializeMap::new();
    let fields = vec![
        Content::I32(100),
        Content::F32(3.14),
    ];

    let mut serializer = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };

    let _ = serializer.end();
    assert!(map.called);
}

