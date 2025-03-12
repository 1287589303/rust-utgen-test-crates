// Answer 0

#[test]
fn test_bad_type_boolean() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Boolean);
}

#[test]
fn test_bad_type_integer() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Integer);
}

#[test]
fn test_bad_type_float() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Float);
}

#[test]
fn test_bad_type_char() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Char);
}

#[test]
fn test_bad_type_string() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::String);
}

#[test]
fn test_bad_type_byte_array() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::ByteArray);
}

#[test]
fn test_bad_type_optional() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Optional);
}

#[test]
fn test_bad_type_sequence() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Sequence);
}

#[test]
fn test_bad_type_tuple() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Tuple);
}

#[test]
fn test_bad_type_tuple_struct() {
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err("Not implemented".to_string()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Err("Not implemented".to_string()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Err("Not implemented".to_string()) }
        fn is_human_readable(&self) -> bool { false }
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: MockSerializer,
    };
    
    let error = serializer.bad_type(Unsupported::TupleStruct);
}

