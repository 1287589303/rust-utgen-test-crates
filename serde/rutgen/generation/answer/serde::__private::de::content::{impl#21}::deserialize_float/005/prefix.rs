// Answer 0

#[test]
fn test_deserialize_float_i32_min() {
    struct TestVisitor;

    impl Visitor<'static> for TestVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i32(self, value: i32) -> Result<Self::Value, Box<dyn std::error::Error>> { assert_eq!(value, -2_147_483_648); Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
    }

    let content = Content::I32(-2_147_483_648);
    let deserializer = ContentRefDeserializer { content: &content, err: std::marker::PhantomData };
    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_i32_max() {
    struct TestVisitor;

    impl Visitor<'static> for TestVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i32(self, value: i32) -> Result<Self::Value, Box<dyn std::error::Error>> { assert_eq!(value, 2_147_483_647); Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
    }

    let content = Content::I32(2_147_483_647);
    let deserializer = ContentRefDeserializer { content: &content, err: std::marker::PhantomData };
    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_i32_middle() {
    struct TestVisitor;

    impl Visitor<'static> for TestVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i32(self, value: i32) -> Result<Self::Value, Box<dyn std::error::Error>> { assert_eq!(value, 0); Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
    }

    let content = Content::I32(0);
    let deserializer = ContentRefDeserializer { content: &content, err: std::marker::PhantomData };
    let _ = deserializer.deserialize_float(TestVisitor);
}

