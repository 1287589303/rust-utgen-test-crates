// Answer 0

#[test]
fn test_deserialize_identifier_string() {
    struct DummyVisitor;
    
    impl Visitor<'static> for DummyVisitor {
        type Value = String;

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }

        fn visit_borrowed_str<E>(self, v: &'static str) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }

        fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected bytes"))
        }

        fn visit_u8<E>(self, _v: u8) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u8"))
        }

        fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u64"))
        }

        fn visit_borrowed_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed bytes"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let cases = vec![
        Content::String("test".to_string()),
        Content::String("".to_string()),
    ];

    for content in cases {
        let deserializer = ContentRefDeserializer {
            content: &content,
            err: std::marker::PhantomData,
        };
        let _ = deserializer.deserialize_identifier(DummyVisitor);
    }
}

#[test]
fn test_deserialize_identifier_str() {
    struct DummyVisitor;
    
    impl Visitor<'static> for DummyVisitor {
        type Value = String;

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }

        fn visit_borrowed_str<E>(self, v: &'static str) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }

        fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected bytes"))
        }

        fn visit_u8<E>(self, _v: u8) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u8"))
        }

        fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u64"))
        }

        fn visit_borrowed_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed bytes"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let cases = vec![
        Content::Str("hello"),
        Content::Str(""),
    ];

    for content in cases {
        let deserializer = ContentRefDeserializer {
            content: &content,
            err: std::marker::PhantomData,
        };
        let _ = deserializer.deserialize_identifier(DummyVisitor);
    }
}

#[test]
fn test_deserialize_identifier_byte_vec() {
    struct DummyVisitor;
    
    impl Visitor<'static> for DummyVisitor {
        type Value = Vec<u8>;

        fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected str"))
        }

        fn visit_borrowed_str<E>(self, _v: &'static str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed str"))
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
            Ok(v.to_vec())
        }

        fn visit_u8<E>(self, _v: u8) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u8"))
        }

        fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u64"))
        }

        fn visit_borrowed_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
            Ok(v.to_vec())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let cases = vec![
        Content::ByteBuf(vec![1, 2, 3]),
        Content::ByteBuf(vec![]),
    ];

    for content in cases {
        let deserializer = ContentRefDeserializer {
            content: &content,
            err: std::marker::PhantomData,
        };
        let _ = deserializer.deserialize_identifier(DummyVisitor);
    }
}

#[test]
fn test_deserialize_identifier_bytes_slice() {
    struct DummyVisitor;
    
    impl Visitor<'static> for DummyVisitor {
        type Value = Vec<u8>;

        fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected str"))
        }

        fn visit_borrowed_str<E>(self, _v: &'static str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed str"))
        }

        fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected bytes"))
        }

        fn visit_u8<E>(self, _v: u8) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u8"))
        }

        fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u64"))
        }

        fn visit_borrowed_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
            Ok(v.to_vec())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let cases = vec![
        Content::Bytes(&[1, 2, 3]),
        Content::Bytes(&[]),
    ];

    for content in cases {
        let deserializer = ContentRefDeserializer {
            content: &content,
            err: std::marker::PhantomData,
        };
        let _ = deserializer.deserialize_identifier(DummyVisitor);
    }
}

#[test]
fn test_deserialize_identifier_u8() {
    struct DummyVisitor;

    impl Visitor<'static> for DummyVisitor {
        type Value = u8;

        fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected str"))
        }

        fn visit_borrowed_str<E>(self, _v: &'static str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed str"))
        }

        fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected bytes"))
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u64"))
        }

        fn visit_borrowed_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed bytes"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let cases = vec![
        Content::U8(0),
        Content::U8(255),
    ];

    for content in cases {
        let deserializer = ContentRefDeserializer {
            content: &content,
            err: std::marker::PhantomData,
        };
        let _ = deserializer.deserialize_identifier(DummyVisitor);
    }
}

#[test]
fn test_deserialize_identifier_u64() {
    struct DummyVisitor;

    impl Visitor<'static> for DummyVisitor {
        type Value = u64;

        fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected str"))
        }

        fn visit_borrowed_str<E>(self, _v: &'static str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed str"))
        }

        fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected bytes"))
        }

        fn visit_u8<E>(self, _v: u8) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected u8"))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_borrowed_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed bytes"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let cases = vec![
        Content::U64(0),
        Content::U64(18_446_744_073_709_551_615),
    ];

    for content in cases {
        let deserializer = ContentRefDeserializer {
            content: &content,
            err: std::marker::PhantomData,
        };
        let _ = deserializer.deserialize_identifier(DummyVisitor);
    }
}

