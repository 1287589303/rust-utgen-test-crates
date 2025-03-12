// Answer 0

#[test]
fn test_deserialize_integer_invalid_type_none() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Err(()) }
        fn visit_str(self, _: &str) -> Result<Self::Value, ()> { Err(()) }
        fn visit_string(self, _: String) -> Result<Self::Value, ()> { Err(()) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, ()> { Err(()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> { Err(()) }
        fn visit_option<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Err(()) }
        fn visit_unit_struct(self, _: &'static str) -> Result<Self::Value, ()> { Err(()) }
        fn visit_newtype_struct<V>(self, _: &'static str, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, ()> where V: SeqAccess<'de> { Err(()) }
        fn visit_tuple<V>(self, _: usize, _: V) -> Result<Self::Value, ()> where V: TupleAccess<'de> { Err(()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, ()> where V: MapAccess<'de> { Err(()) }
        fn visit_struct<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<Self::Value, ()> where V: MapAccess<'de> { Err(()) }
    }
    
    let content = Content::None;
    let deserializer = ContentDeserializer::<()>{ content, err: PhantomData };
    let _ = deserializer.deserialize_integer(TestVisitor);
}

#[test]
fn test_deserialize_integer_invalid_type_bytes() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Err(()) }
        fn visit_str(self, _: &str) -> Result<Self::Value, ()> { Err(()) }
        fn visit_string(self, _: String) -> Result<Self::Value, ()> { Err(()) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, ()> { Err(()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> { Err(()) }
        fn visit_option<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Err(()) }
        fn visit_unit_struct(self, _: &'static str) -> Result<Self::Value, ()> { Err(()) }
        fn visit_newtype_struct<V>(self, _: &'static str, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, ()> where V: SeqAccess<'de> { Err(()) }
        fn visit_tuple<V>(self, _: usize, _: V) -> Result<Self::Value, ()> where V: TupleAccess<'de> { Err(()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, ()> where V: MapAccess<'de> { Err(()) }
        fn visit_struct<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<Self::Value, ()> where V: MapAccess<'de> { Err(()) }
    }

    let content = Content::Bytes(vec![0u8, 1u8, 2u8]);
    let deserializer = ContentDeserializer::<()>{ content, err: PhantomData };
    let _ = deserializer.deserialize_integer(TestVisitor);
}

#[test]
fn test_deserialize_integer_invalid_type_newtype() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Err(()) }
        fn visit_str(self, _: &str) -> Result<Self::Value, ()> { Err(()) }
        fn visit_string(self, _: String) -> Result<Self::Value, ()> { Err(()) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, ()> { Err(()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> { Err(()) }
        fn visit_option<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Err(()) }
        fn visit_unit_struct(self, _: &'static str) -> Result<Self::Value, ()> { Err(()) }
        fn visit_newtype_struct<V>(self, _: &'static str, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, ()> where V: SeqAccess<'de> { Err(()) }
        fn visit_tuple<V>(self, _: usize, _: V) -> Result<Self::Value, ()> where V: TupleAccess<'de> { Err(()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, ()> where V: MapAccess<'de> { Err(()) }
        fn visit_struct<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<Self::Value, ()> where V: MapAccess<'de> { Err(()) }
    }

    let content = Content::NewtypeStruct("newtype", Box::new(Content::None));
    let deserializer = ContentDeserializer::<()>{ content, err: PhantomData };
    let _ = deserializer.deserialize_integer(TestVisitor);
}

