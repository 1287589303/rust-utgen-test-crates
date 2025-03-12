// Answer 0

#[test]
fn test_serialize_human_readable_case() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

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
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Ok(()); }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> { 
            assert_eq!(v, "1001:1002:1003:1004:1005:1006:1007:1008");
            Ok(()) 
        }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn is_human_readable(&self) -> bool { true }
    }

    let data = "1001:1002:1003:1004:1005:1006:1007:1008";
    let serializer = TestSerializer;
    data.serialize(serializer).unwrap();
}

