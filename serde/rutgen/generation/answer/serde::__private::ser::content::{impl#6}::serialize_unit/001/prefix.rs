// Answer 0

#[test]
fn test_serialize_unit_with_content_serializer() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_unit();
}

#[test]
fn test_serialize_unit_with_custom_error_serializer() {
    struct CustomError;
    impl ser::Error for CustomError {}

    let serializer = ContentSerializer::<CustomError>::default();
    let _ = serializer.serialize_unit();
}

