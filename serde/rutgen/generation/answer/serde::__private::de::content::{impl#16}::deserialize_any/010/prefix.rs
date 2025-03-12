// Answer 0

#[test]
fn test_deserialize_any_string_non_empty() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_string(self, _: String) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_borrowed_str(self, _: &'_ str) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_borrowed_bytes(self, _: &'_ [u8]) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_some<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_newtype_struct<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_seq<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_map<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
    }

    let content = Content::String("test".to_string());
    let deserializer = ContentDeserializer::new(content);
    let _ = deserializer.deserialize_any(VisitorImpl);
}

#[test]
fn test_deserialize_any_string_empty() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_string(self, _: String) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_borrowed_str(self, _: &'_ str) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_borrowed_bytes(self, _: &'_ [u8]) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_some<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_newtype_struct<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_seq<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_map<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
    }

    let content = Content::String("".to_string());
    let deserializer = ContentDeserializer::new(content);
    let _ = deserializer.deserialize_any(VisitorImpl);
}

#[test]
fn test_deserialize_any_string_longer() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_string(self, _: String) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_borrowed_str(self, _: &'_ str) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_borrowed_bytes(self, _: &'_ [u8]) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_some<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_newtype_struct<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_seq<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_map<V: Deserialize>(self, _: V) -> Result<Self::Value, ()> { Ok(()) }
    }

    let content = Content::String("longer string test".to_string());
    let deserializer = ContentDeserializer::new(content);
    let _ = deserializer.deserialize_any(VisitorImpl);
}

