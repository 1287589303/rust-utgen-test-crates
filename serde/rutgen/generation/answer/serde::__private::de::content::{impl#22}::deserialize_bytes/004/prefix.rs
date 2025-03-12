// Answer 0

#[test]
fn test_deserialize_bytes_byte_buf() {
    struct MockVisitor;
    
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_bytes(self, value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value.to_vec())
        }

        fn visit_str(self, _: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }
        
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }
    }

    let content = Content::ByteBuf(vec![1, 2, 3]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_bytes(MockVisitor);
}

#[test]
fn test_deserialize_bytes_string() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_str(self, value: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value.to_string())
        }

        fn visit_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }
        
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }
    }

    let content = Content::String("test".to_string());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_bytes(MockVisitor);
}

#[test]
fn test_deserialize_bytes_str() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value)
        }

        fn visit_str(self, _: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }
    }

    let content = Content::Str("test str");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_bytes(MockVisitor);
}

#[test]
fn test_deserialize_bytes_bytes() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value.to_vec())
        }

        fn visit_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_str(self, _: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }
    }

    let content = Content::Bytes(vec![10, 20, 30]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_bytes(MockVisitor);
}

#[test]
fn test_deserialize_bytes_seq() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, Box<dyn std::error::Error>>
        where
            V: SeqAccess<'de>,
        {
            let mut bytes = Vec::new();
            while let Some(value) = seq.next_element::<u8>()? {
                bytes.push(value);
            }
            Ok(bytes)
        }

        fn visit_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_str(self, _: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected call".into())
        }
    }

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_bytes(MockVisitor);
}

