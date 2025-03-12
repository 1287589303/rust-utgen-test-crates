// Answer 0

#[test]
fn test_content_serializer_new() {
    let serializer: ContentSerializer<()> = ContentSerializer::new();
}

#[test]
fn test_content_serializer_new_with_different_error_type() {
    struct DummyError;
    let serializer: ContentSerializer<DummyError> = ContentSerializer::new();
}

