// Answer 0

#[test]
fn test_deserialize_char_invalid_type_i32() {
    let visitor = MockVisitor::new();
    let content = Content::I32(42);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_char(visitor);
}

#[test]
fn test_deserialize_char_invalid_type_f32() {
    let visitor = MockVisitor::new();
    let content = Content::F32(3.14);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_char(visitor);
}

#[test]
fn test_deserialize_char_invalid_type_seq() {
    let visitor = MockVisitor::new();
    let content = Content::Seq(vec![Content::I32(1), Content::I32(2)]);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_char(visitor);
}

#[test]
fn test_deserialize_char_invalid_type_map() {
    let visitor = MockVisitor::new();
    let content = Content::Map(vec![(Content::Str("key"), Content::I32(2))]);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_char(visitor);
}

