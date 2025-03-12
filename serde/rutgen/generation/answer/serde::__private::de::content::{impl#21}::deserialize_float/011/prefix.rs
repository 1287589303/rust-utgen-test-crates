// Answer 0

#[test]
fn test_deserialize_float_u8() {
    struct VisitorMock {
        value: Option<f32>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = f32;

        fn visit_f32(self, value: f32) -> Result<Self::Value, ()> {
            self.value = Some(value);
            Ok(value)
        }

        /* Implement other visit methods as needed, but for this test only f32 is necessary */
        fn visit_u8(self, value: u8) -> Result<Self::Value, ()> {
            self.value = Some(value as f32);
            Ok(value as f32)
        }
    }

    let content = Content::U8(128);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = VisitorMock { value: None };
    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_f32() {
    struct VisitorMock {
        value: Option<f32>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = f32;

        fn visit_f32(self, value: f32) -> Result<Self::Value, ()> {
            self.value = Some(value);
            Ok(value)
        }
    }

    let content = Content::F32(1.0);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = VisitorMock { value: None };
    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_f64() {
    struct VisitorMock {
        value: Option<f32>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = f32;

        fn visit_f64(self, value: f64) -> Result<Self::Value, ()> {
            self.value = Some(value as f32);
            Ok(value as f32)
        }
    }

    let content = Content::F64(2.0);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = VisitorMock { value: None };
    let _ = deserializer.deserialize_float(visitor);
}

