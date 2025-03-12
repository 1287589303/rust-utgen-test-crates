// Answer 0

#[test]
fn test_deserialize_float_i8_min() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
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

    let content = Content::I8(-128);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = DummyVisitor;

    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_i8_max() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
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

    let content = Content::I8(127);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = DummyVisitor;

    let _ = deserializer.deserialize_float(visitor);
}

