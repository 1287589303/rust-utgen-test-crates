// Answer 0

#[test]
fn test_deserialize_integer_with_none() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = Result<(), ()>;
        fn visit_u8(self, _: u8) -> Self::Value { Err(()) }
        fn visit_u16(self, _: u16) -> Self::Value { Err(()) }
        fn visit_u32(self, _: u32) -> Self::Value { Err(()) }
        fn visit_u64(self, _: u64) -> Self::Value { Err(()) }
        fn visit_i8(self, _: i8) -> Self::Value { Err(()) }
        fn visit_i16(self, _: i16) -> Self::Value { Err(()) }
        fn visit_i32(self, _: i32) -> Self::Value { Err(()) }
        fn visit_i64(self, _: i64) -> Self::Value { Err(()) }
    }

    let content = Content::None;
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(VisitorImpl);
}

#[test]
fn test_deserialize_integer_with_bool() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = Result<(), ()>;
        fn visit_u8(self, _: u8) -> Self::Value { Err(()) }
        fn visit_u16(self, _: u16) -> Self::Value { Err(()) }
        fn visit_u32(self, _: u32) -> Self::Value { Err(()) }
        fn visit_u64(self, _: u64) -> Self::Value { Err(()) }
        fn visit_i8(self, _: i8) -> Self::Value { Err(()) }
        fn visit_i16(self, _: i16) -> Self::Value { Err(()) }
        fn visit_i32(self, _: i32) -> Self::Value { Err(()) }
        fn visit_i64(self, _: i64) -> Self::Value { Err(()) }
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(VisitorImpl);
}

#[test]
fn test_deserialize_integer_with_f32() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = Result<(), ()>;
        fn visit_u8(self, _: u8) -> Self::Value { Err(()) }
        fn visit_u16(self, _: u16) -> Self::Value { Err(()) }
        fn visit_u32(self, _: u32) -> Self::Value { Err(()) }
        fn visit_u64(self, _: u64) -> Self::Value { Err(()) }
        fn visit_i8(self, _: i8) -> Self::Value { Err(()) }
        fn visit_i16(self, _: i16) -> Self::Value { Err(()) }
        fn visit_i32(self, _: i32) -> Self::Value { Err(()) }
        fn visit_i64(self, _: i64) -> Self::Value { Err(()) }
    }

    let content = Content::F32(3.14);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(VisitorImpl);
}

#[test]
fn test_deserialize_integer_with_f64() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = Result<(), ()>;
        fn visit_u8(self, _: u8) -> Self::Value { Err(()) }
        fn visit_u16(self, _: u16) -> Self::Value { Err(()) }
        fn visit_u32(self, _: u32) -> Self::Value { Err(()) }
        fn visit_u64(self, _: u64) -> Self::Value { Err(()) }
        fn visit_i8(self, _: i8) -> Self::Value { Err(()) }
        fn visit_i16(self, _: i16) -> Self::Value { Err(()) }
        fn visit_i32(self, _: i32) -> Self::Value { Err(()) }
        fn visit_i64(self, _: i64) -> Self::Value { Err(()) }
    }

    let content = Content::F64(2.71828);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(VisitorImpl);
}

#[test]
fn test_deserialize_integer_with_string() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = Result<(), ()>;
        fn visit_u8(self, _: u8) -> Self::Value { Err(()) }
        fn visit_u16(self, _: u16) -> Self::Value { Err(()) }
        fn visit_u32(self, _: u32) -> Self::Value { Err(()) }
        fn visit_u64(self, _: u64) -> Self::Value { Err(()) }
        fn visit_i8(self, _: i8) -> Self::Value { Err(()) }
        fn visit_i16(self, _: i16) -> Self::Value { Err(()) }
        fn visit_i32(self, _: i32) -> Self::Value { Err(()) }
        fn visit_i64(self, _: i64) -> Self::Value { Err(()) }
    }

    let content = Content::String("some string".to_string());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(VisitorImpl);
}

#[test]
fn test_deserialize_integer_with_char() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = Result<(), ()>;
        fn visit_u8(self, _: u8) -> Self::Value { Err(()) }
        fn visit_u16(self, _: u16) -> Self::Value { Err(()) }
        fn visit_u32(self, _: u32) -> Self::Value { Err(()) }
        fn visit_u64(self, _: u64) -> Self::Value { Err(()) }
        fn visit_i8(self, _: i8) -> Self::Value { Err(()) }
        fn visit_i16(self, _: i16) -> Self::Value { Err(()) }
        fn visit_i32(self, _: i32) -> Self::Value { Err(()) }
        fn visit_i64(self, _: i64) -> Self::Value { Err(()) }
    }

    let content = Content::Char('a');
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(VisitorImpl);
}

