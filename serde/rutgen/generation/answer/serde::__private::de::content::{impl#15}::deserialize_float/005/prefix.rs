// Answer 0

#[test]
fn test_deserialize_float_with_i32_max() {
    struct VisitorStub;
    impl Visitor<'_> for VisitorStub {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i32(self, v: i32) -> Result<Self::Value, E> { assert_eq!(v, 2147483647); Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { Ok(()) }
    }

    let content = Content::I32(2147483647);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = VisitorStub;

    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_with_i32_zero() {
    struct VisitorStub;
    impl Visitor<'_> for VisitorStub {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i32(self, v: i32) -> Result<Self::Value, E> { assert_eq!(v, 0); Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { Ok(()) }
    }

    let content = Content::I32(0);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = VisitorStub;

    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_with_i32_min() {
    struct VisitorStub;
    impl Visitor<'_> for VisitorStub {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i32(self, v: i32) -> Result<Self::Value, E> { assert_eq!(v, -2147483648); Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { Ok(()) }
    }

    let content = Content::I32(-2147483648);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = VisitorStub;

    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_with_invalid_type_f32() {
    struct VisitorStub;
    impl Visitor<'_> for VisitorStub {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { Ok(()) }
    }

    let content = Content::F32(1.0);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = VisitorStub;

    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_with_invalid_type_f64() {
    struct VisitorStub;
    impl Visitor<'_> for VisitorStub {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { Ok(()) }
    }

    let content = Content::F64(1.0);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = VisitorStub;

    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_with_invalid_type_u8() {
    struct VisitorStub;
    impl Visitor<'_> for VisitorStub {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, E> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { Ok(()) }
    }

    let content = Content::U8(1);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = VisitorStub;

    let _ = deserializer.deserialize_float(visitor);
}

