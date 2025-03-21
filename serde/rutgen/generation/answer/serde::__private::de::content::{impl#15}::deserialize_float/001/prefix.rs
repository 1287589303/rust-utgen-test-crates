// Answer 0

#[test]
fn test_deserialize_float_with_char() {
    struct MockVisitor;

    impl Visitor<'_> for MockVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_char(self, _: char) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::Char('a');
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };
    let _ = deserializer.deserialize_float(MockVisitor);
}

#[test]
fn test_deserialize_float_with_string() {
    struct MockVisitor;

    impl Visitor<'_> for MockVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_string(self, _: String) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::String(String::from("test"));
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };
    let _ = deserializer.deserialize_float(MockVisitor);
}

#[test]
fn test_deserialize_float_with_unit() {
    struct MockVisitor;

    impl Visitor<'_> for MockVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_unit(self) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::Unit;
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };
    let _ = deserializer.deserialize_float(MockVisitor);
}

