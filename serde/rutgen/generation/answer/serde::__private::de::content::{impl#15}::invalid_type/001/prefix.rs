// Answer 0

#[test]
fn test_invalid_type_with_bool_content() {
    struct MockExpected;
    impl Expected for MockExpected {}
    
    let content = Content::Bool(true);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;
    
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_with_u8_content() {
    struct MockExpected;
    impl Expected for MockExpected {}
    
    let content = Content::U8(255);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_with_i32_content() {
    struct MockExpected;
    impl Expected for MockExpected {}

    let content = Content::I32(-2147483648);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_with_f32_content() {
    struct MockExpected;
    impl Expected for MockExpected {}

    let content = Content::F32(3.14);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_with_char_content() {
    struct MockExpected;
    impl Expected for MockExpected {}

    let content = Content::Char('A');
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_with_string_content() {
    struct MockExpected;
    impl Expected for MockExpected {}

    let content = Content::String("test string".to_string());
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
} 

#[test]
fn test_invalid_type_with_bytes_content() {
    struct MockExpected;
    impl Expected for MockExpected {}

    let content = Content::Bytes(vec![1, 2, 3]);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_with_seq_content() {
    struct MockExpected;
    impl Expected for MockExpected {}

    let content = Content::Seq(vec![Content::U8(1), Content::Bool(false)]);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_with_map_content() {
    struct MockExpected;
    impl Expected for MockExpected {}

    let content = Content::Map(vec![(Content::Str("key"), Content::U8(2))]);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let exp = MockExpected;

    let _ = deserializer.invalid_type(&exp);
}

