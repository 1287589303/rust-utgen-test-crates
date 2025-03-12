// Answer 0

#[test]
fn test_end_success_with_non_empty_fields() {
    struct MockMap {
        result: Result<(), MockError>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = MockError;
        
        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            self.result.clone()
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    struct MockError;

    let mut map = MockMap { result: Ok(()) };
    let name: &'static str = "test_variant";
    let fields = vec![Content::U8(10), Content::String("test".to_string())];

    let serializer = SerializeTupleVariantAsMapValue {
        map,
        name,
        fields,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_success_with_non_empty_fields_and_different_content() {
    struct MockMap {
        result: Result<(), MockError>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = MockError;
        
        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            self.result.clone()
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    struct MockError;

    let mut map = MockMap { result: Ok(()) };
    let name: &'static str = "another_variant";
    let fields = vec![Content::F64(1.23), Content::Bool(true)];

    let serializer = SerializeTupleVariantAsMapValue {
        map,
        name,
        fields,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_with_failure_on_serialize_value() {
    struct MockMap {
        result: Result<(), MockError>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = MockError;
        
        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            self.result.clone()
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    struct MockError;

    let mut map = MockMap { result: Err(MockError) };
    let name: &'static str = "error_variant";
    let fields = vec![Content::I32(42)];

    let serializer = SerializeTupleVariantAsMapValue {
        map,
        name,
        fields,
    };

    let _ = serializer.end();
}

