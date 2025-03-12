// Answer 0

#[test]
fn test_serialize_struct_ok() {
    struct TestSerializer;
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = &'static str;
        type SerializeSeq = TestSeq;
        type SerializeTuple = TestTuple;
        type SerializeTupleStruct = TestTupleStruct;
        type SerializeTupleVariant = TestTupleVariant;
        type SerializeMap = TestMap;
        type SerializeStruct = TestStruct;
        type SerializeStructVariant = TestStructVariant;

        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(TestStruct)
        }

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Ok(TestTuple) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Ok(TestStructVariant) }
        fn collect_seq<I>(self, _: I) -> Result<Self::Ok, Self::Error> where I: IntoIterator, <I as IntoIterator>::Item: Serialize { Ok(()) }
        fn collect_map<K, V, I>(self, _: I) -> Result<Self::Ok, Self::Error> where K: Serialize, V: Serialize, I: IntoIterator<Item = (K, V)> { Ok(()) }
        fn collect_str<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + std::fmt::Display { Ok(()) }
        fn is_human_readable(&self) -> bool { false }
    }

    struct TestStruct;
    impl SerializeStruct for TestStruct {
        type Ok = ();
        type Error = &'static str;

        fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize { Ok(()) }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    struct TestTuple;
    struct TestTupleStruct;
    struct TestTupleVariant;
    struct TestMap;
    struct TestStructVariant;

    struct RangeFrom {
        start: i32,
    }

    let range_from = RangeFrom { start: 1 };
    let _ = range_from.serialize(TestSerializer);
}

#[test]
fn test_serialize_field_err() {
    struct FailingSerializer;
    impl Serializer for FailingSerializer {
        type Ok = ();
        type Error = &'static str;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = FailingStruct;
        type SerializeStructVariant = ();

        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(FailingStruct)
        }

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn collect_seq<I>(self, _: I) -> Result<Self::Ok, Self::Error> where I: IntoIterator, <I as IntoIterator>::Item: Serialize { Ok(()) }
        fn collect_map<K, V, I>(self, _: I) -> Result<Self::Ok, Self::Error> where K: Serialize, V: Serialize, I: IntoIterator<Item = (K, V)> { Ok(()) }
        fn is_human_readable(&self) -> bool { false }
    }

    struct FailingStruct;
    impl SerializeStruct for FailingStruct {
        type Ok = ();
        type Error = &'static str;

        fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize {
            Err("serialization failed")
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    struct RangeFrom {
        start: i32,
    }

    let range_from = RangeFrom { start: 1 };
    let _ = range_from.serialize(FailingSerializer);
}

