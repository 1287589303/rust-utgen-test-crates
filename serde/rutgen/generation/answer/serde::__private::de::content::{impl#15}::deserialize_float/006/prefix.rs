// Answer 0

#[test]
fn test_deserialize_float_with_i16_min() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = i16;

        fn visit_i16(self, value: i16) -> Result<Self::Value, ()> {
            Ok(value)
        }

        fn visit_f32(self, _value: f32) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_f64(self, _value: f64) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_u8(self, _value: u8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u16(self, _value: u16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u32(self, _value: u32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u64(self, _value: u64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i8(self, _value: i8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i32(self, _value: i32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i64(self, _value: i64) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::I16(-32_768);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let _ = deserializer.deserialize_float(DummyVisitor);
}

#[test]
fn test_deserialize_float_with_i16_max() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = i16;

        fn visit_i16(self, value: i16) -> Result<Self::Value, ()> {
            Ok(value)
        }

        fn visit_f32(self, _value: f32) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_f64(self, _value: f64) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_u8(self, _value: u8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u16(self, _value: u16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u32(self, _value: u32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u64(self, _value: u64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i8(self, _value: i8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i32(self, _value: i32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i64(self, _value: i64) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::I16(32_767);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let _ = deserializer.deserialize_float(DummyVisitor);
}

#[test]
fn test_deserialize_float_with_i16_mid() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = i16;

        fn visit_i16(self, value: i16) -> Result<Self::Value, ()> {
            Ok(value)
        }

        fn visit_f32(self, _value: f32) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_f64(self, _value: f64) -> Result<Self::Value, ()> {
            Err(())
        }
        
        fn visit_u8(self, _value: u8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u16(self, _value: u16) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u32(self, _value: u32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u64(self, _value: u64) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i8(self, _value: i8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i32(self, _value: i32) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_i64(self, _value: i64) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::I16(0);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let _ = deserializer.deserialize_float(DummyVisitor);
}

