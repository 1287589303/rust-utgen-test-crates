// Answer 0

#[test]
fn test_deserialize_float_with_i64_min() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Err(E) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Err(E) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Err(E) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Err(E) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Err(E) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Err(E) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Err(E) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Err(E) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, E> { Err(E) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { }
    }

    let content = Content::I64(-9_223_372_036_854_775_808);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    deserializer.deserialize_float(MockVisitor);
}

#[test]
fn test_deserialize_float_with_i64_max() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();
        fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Err(E) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Err(E) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Err(E) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Err(E) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Err(E) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Err(E) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Err(E) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Err(E) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, E> { Err(E) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, E> { }
    }

    let content = Content::I64(9_223_372_036_854_775_807);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    deserializer.deserialize_float(MockVisitor);
}

