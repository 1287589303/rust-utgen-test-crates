// Answer 0

#[test]
fn test_serialize_some_bool() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = true;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_i8() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: i8 = 127;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_i16() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: i16 = 32767;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_i32() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: i32 = 2147483647;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_i64() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: i64 = 9223372036854775807;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_i128() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: i128 = 170141183460469231731687303715884105727;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_u8() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: u8 = 255;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_u16() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: u16 = 65535;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_u32() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: u32 = 4294967295;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_u64() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: u64 = 18446744073709551615;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_u128() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: u128 = 340282366920938463463374607431768211455;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_f32() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: f32 = 3.4028235e38;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_f64() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: f64 = 1.7976931348623157e308;
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_char() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value: char = 'a';
    serializer.serialize_some(&value).unwrap();
}

#[test]
fn test_serialize_some_struct() {
    #[derive(Serialize)]
    struct TestStruct {
        field: String,
    }
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = TestStruct { field: String::from("test") };
    serializer.serialize_some(&value).unwrap();
}

