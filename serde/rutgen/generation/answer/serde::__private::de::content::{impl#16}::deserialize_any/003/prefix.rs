// Answer 0

#[test]
fn test_deserialize_newtype_bool() {
    let content = Content::Newtype(Box::new(Content::Bool(true)));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_newtype_u8() {
    let content = Content::Newtype(Box::new(Content::U8(255)));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_newtype_u16() {
    let content = Content::Newtype(Box::new(Content::U16(65535)));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_newtype_i8() {
    let content = Content::Newtype(Box::new(Content::I8(-128)));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_newtype_f32() {
    let content = Content::Newtype(Box::new(Content::F32(3.14)));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_newtype_string() {
    let content = Content::Newtype(Box::new(Content::String("test".to_string())));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_newtype_bytes() {
    let content = Content::Newtype(Box::new(Content::Bytes(vec![1, 2, 3])));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_newtype_char() {
    let content = Content::Newtype(Box::new(Content::Char('c')));
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor::default();
    let _ = deserializer.deserialize_any(visitor);
}

