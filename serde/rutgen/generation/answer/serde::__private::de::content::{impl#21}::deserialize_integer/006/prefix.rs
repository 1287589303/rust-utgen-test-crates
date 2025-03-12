// Answer 0

#[test]
fn test_deserialize_integer_u8() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = ();

        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Ok(())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u8".into())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u8".into())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u8".into())
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u8".into())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u8".into())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u8".into())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u8".into())
        }
    }

    let content = Content::U8(255);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_integer(TestVisitor);
}

#[test]
fn test_deserialize_integer_u64() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = ();

        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u64".into())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u64".into())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u64".into())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Ok(())
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u64".into())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u64".into())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u64".into())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected u64".into())
        }
    }

    let content = Content::U64(18446744073709551615);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_integer(TestVisitor);
} 

#[test]
fn test_deserialize_integer_i64() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = ();

        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected i64".into())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected i64".into())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected i64".into())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected i64".into())
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected i64".into())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected i64".into())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Err("Expected i64".into())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::fmt::Debug>> {
            Ok(())
        }
    }

    let content = Content::I64(-9223372036854775808);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_integer(TestVisitor);
}

