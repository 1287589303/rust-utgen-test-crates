// Answer 0

#[test]
fn test_serialize_i32_negative_boundary() {
    struct DummySerializer;

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeMap = Impossible<(), ()>;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unimplemented!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
    }

    let serializer = DummySerializer;
    let _ = serializer.serialize_i32(-2147483648);
}

#[test]
fn test_serialize_i32_negative() {
    struct DummySerializer;

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeMap = Impossible<(), ()>;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unimplemented!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
    }

    let serializer = DummySerializer;
    let _ = serializer.serialize_i32(-1);
}

#[test]
fn test_serialize_i32_zero() {
    struct DummySerializer;

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeMap = Impossible<(), ()>;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unimplemented!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
    }

    let serializer = DummySerializer;
    let _ = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_positive() {
    struct DummySerializer;

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeMap = Impossible<(), ()>;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unimplemented!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
    }

    let serializer = DummySerializer;
    let _ = serializer.serialize_i32(2147483647);
}

