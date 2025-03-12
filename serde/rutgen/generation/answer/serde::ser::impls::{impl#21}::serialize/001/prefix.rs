// Answer 0

#[test]
fn test_serialize_struct_error() {
    struct ErroneousSerializer;

    impl Serializer for ErroneousSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ErroneousStruct;
        type SerializeStructVariant = ();

        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Err("Serialize error".into())
        }

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> { Ok(()) }

        fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }
        
        fn is_human_readable(&self) -> bool { false }
    }

    struct ErroneousStruct;

    impl SerializeStruct for ErroneousStruct {
        type Ok = ();
        type Error = String;

        fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize 
        {
            Err("Field serialization error".into())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    struct Data {
        secs: u64,
        nanos: u32,
    }

    impl Serialize for Data {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            let mut state = serializer.serialize_struct("Duration", 2)?;
            state.serialize_field("secs", &self.secs)?;
            state.serialize_field("nanos", &self.nanos)?;
            state.end()
        }
    }

    let data = Data { secs: 1, nanos: 2 };
    let serializer = ErroneousSerializer;
    let result = data.serialize(serializer);
}

