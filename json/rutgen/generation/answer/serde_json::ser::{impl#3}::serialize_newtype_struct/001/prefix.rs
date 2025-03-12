// Answer 0

#[test]
fn test_serialize_newtype_struct_empty_struct() {
    struct EmptyStruct;

    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = EmptyStruct;

    serializer.serialize_newtype_struct("empty_struct", &value);
}

#[test]
fn test_serialize_newtype_struct_large_struct() {
    struct LargeStruct {
        a: i32,
        b: String,
        c: Vec<u8>,
        d: f64,
    }

    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = LargeStruct {
        a: 42,
        b: "Hello".to_string(),
        c: vec![1, 2, 3],
        d: 3.14,
    };

    serializer.serialize_newtype_struct("large_struct", &value);
}

#[test]
fn test_serialize_newtype_struct_string() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = "test string";

    serializer.serialize_newtype_struct("string", &value);
}

#[test]
fn test_serialize_newtype_struct_integer() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = 123;

    serializer.serialize_newtype_struct("integer", &value);
}

#[test]
fn test_serialize_newtype_struct_float() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = 3.14159;

    serializer.serialize_newtype_struct("float", &value);
}

