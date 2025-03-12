// Answer 0

#[test]
fn test_end_with_simple_fields() {
    struct TestError;
    impl ser::Error for TestError {}

    let fields = vec![
        ("field1", Content::Bool(true)),
        ("field2", Content::U32(42)),
    ];

    let serializer = SerializeStructVariant {
        name: "TestStruct",
        variant_index: 0,
        variant: "TestVariant",
        fields,
        error: PhantomData::<TestError>,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_with_no_fields() {
    struct TestError;
    impl ser::Error for TestError {}

    let fields = Vec::new();

    let serializer = SerializeStructVariant {
        name: "EmptyStruct",
        variant_index: 1,
        variant: "EmptyVariant",
        fields,
        error: PhantomData::<TestError>,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_with_multiple_fields() {
    struct TestError;
    impl ser::Error for TestError {}

    let fields = vec![
        ("fieldA", Content::String("Hello".to_string())),
        ("fieldB", Content::F64(3.14)),
        ("fieldC", Content::Unit),
    ];

    let serializer = SerializeStructVariant {
        name: "MultipleFieldsStruct",
        variant_index: 2,
        variant: "MultipleFieldsVariant",
        fields,
        error: PhantomData::<TestError>,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_with_boundary_variant_index() {
    struct TestError;
    impl ser::Error for TestError {}

    let fields = vec![
        ("fieldX", Content::Some(Box::new(Content::U8(255)))),
    ];

    let serializer = SerializeStructVariant {
        name: "BoundaryVariantStruct",
        variant_index: 0, // boundary at 0
        variant: "BoundaryVariant",
        fields,
        error: PhantomData::<TestError>,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_with_large_variant_index() {
    struct TestError;
    impl ser::Error for TestError {}

    let fields = vec![
        ("fieldLarge", Content::I32(-1)),
    ];

    let serializer = SerializeStructVariant {
        name: "LargeIndexStruct",
        variant_index: 100, // large index
        variant: "LargeIndexVariant",
        fields,
        error: PhantomData::<TestError>,
    };

    let _ = serializer.end();
}

