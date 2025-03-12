// Answer 0

#[test]
fn test_invalid_type_bool() {
    struct ExpectedBool;
    
    impl Expected for ExpectedBool {}

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedBool;
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_u8() {
    struct ExpectedU8;

    impl Expected for ExpectedU8 {}

    let content = Content::U8(255);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedU8;
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_i32() {
    struct ExpectedI32;

    impl Expected for ExpectedI32 {}

    let content = Content::I32(-100);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedI32;
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_f64() {
    struct ExpectedF64;

    impl Expected for ExpectedF64 {}

    let content = Content::F64(3.14);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedF64;
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_char() {
    struct ExpectedChar;

    impl Expected for ExpectedChar {}

    let content = Content::Char('a');
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedChar;
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_string() {
    struct ExpectedString;

    impl Expected for ExpectedString {}

    let content = Content::String("test".to_string());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedString;
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_seq() {
    struct ExpectedSeq;

    impl Expected for ExpectedSeq {}

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedSeq;
    let _ = deserializer.invalid_type(&exp);
}

#[test]
fn test_invalid_type_map() {
    struct ExpectedMap;

    impl Expected for ExpectedMap {}

    let content = Content::Map(vec![(Content::Str("key"), Content::Str("value"))]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    
    let exp = ExpectedMap;
    let _ = deserializer.invalid_type(&exp);
}

