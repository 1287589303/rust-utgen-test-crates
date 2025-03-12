// Answer 0

#[test]
fn test_deserialize_any_with_seq() {
    use crate::de::Visitor;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_str(self, _: &str) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Ok(()) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Ok(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Ok(()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Ok(()) }
    }

    let content_seq = vec![
        Content::Bool(true),
        Content::U8(255),
        Content::I32(-42),
        Content::F64(3.14),
        Content::Char('c'),
        Content::String("test".to_string()),
        Content::Bytes(vec![1, 2, 3]),
    ];

    let content = Content::Seq(content_seq);

    let deserializer = ContentRefDeserializer::new(&content);
    let _ = deserializer.deserialize_any(TestVisitor);
}

