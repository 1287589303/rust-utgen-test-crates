// Answer 0

#[test]
fn test_end_with_non_empty_name_and_fields() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let name: &'static str = "TestStruct";
    let fields: Vec<Content> = vec![
        Content::Bool(true),
        Content::U8(255),
        Content::String("Hello".to_string()),
    ];

    let serializer = SerializeTupleStruct {
        name,
        fields,
        error: PhantomData::<DummyError>,
    };

    let _result = serializer.end();
}

#[test]
fn test_end_with_one_field() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let name: &'static str = "SingleFieldStruct";
    let fields: Vec<Content> = vec![Content::I32(42)];

    let serializer = SerializeTupleStruct {
        name,
        fields,
        error: PhantomData::<DummyError>,
    };

    let _result = serializer.end();
}

#[test]
fn test_end_with_various_fields() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let name: &'static str = "VariedFieldsStruct";
    let fields: Vec<Content> = vec![
        Content::F64(3.14),
        Content::Char('a'),
        Content::ByteBuf(vec![1, 2, 3]),
    ];

    let serializer = SerializeTupleStruct {
        name,
        fields,
        error: PhantomData::<DummyError>,
    };

    let _result = serializer.end();
}

