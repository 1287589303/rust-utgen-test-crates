// Answer 0

#[test]
#[should_panic]
fn test_end_with_error_on_non_empty_seq() {
    struct TestMap;

    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<Self::Ok, Self::Error> {
            Err(Error)
        }
    }

    let mut map = TestMap;
    let mut fields = Vec::new();
    fields.push(Content::Bool(true));
    fields.push(Content::U32(42));

    let variant = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };
    
    let _ = variant.end();
}

#[test]
#[should_panic]
fn test_end_with_error_on_empty_seq() {
    struct TestMap;

    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<Self::Ok, Self::Error> {
            Err(Error)
        }
    }

    let mut map = TestMap;
    let fields: Vec<Content> = Vec::new();

    let variant = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };

    let _ = variant.end();
}

