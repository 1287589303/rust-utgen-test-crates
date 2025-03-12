// Answer 0

#[test]
fn test_end_with_err_on_serialize_value() {
    struct MockMap {
        should_return_err: bool,
    }

    impl SerializeMap for MockMap {
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            if self.should_return_err {
                Err(Error)
            } else {
                Ok(())
            }
        }
    }

    let name: &'static str = "test_struct";
    let fields = vec![("field1", Content::Bool(true))];
    let mut map = MockMap { should_return_err: true };
    let mut serializer = FlatMapSerializeStructVariantAsMapValue {
        map: &mut map,
        name,
        fields,
    };

    let _ = serializer.end(); // This should trigger the Err path
}

#[test]
fn test_end_with_non_empty_fields() {
    struct MockMap {
        should_return_err: bool,
    }

    impl SerializeMap for MockMap {
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            if self.should_return_err {
                Err(Error)
            } else {
                Ok(())
            }
        }
    }

    let name: &'static str = "test_struct";
    let fields = vec![("field1", Content::U8(10))];
    let mut map = MockMap { should_return_err: true };
    let mut serializer = FlatMapSerializeStructVariantAsMapValue {
        map: &mut map,
        name,
        fields,
    };

    let _ = serializer.end(); // This should trigger the Err path
}

