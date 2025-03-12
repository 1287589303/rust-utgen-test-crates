// Answer 0

#[test]
fn test_serialize_when_human_readable_false_with_valid_octets() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = std::io::Error;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn octets(&self) -> &[u8] {
            b"valid_byte_slice"
        }

        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                // This block won't execute
                Ok(())
            } else {
                self.octets().serialize(serializer)
            }
        }
    }

    let test_value = TestStruct;
    let serializer = TestSerializer;

    test_value.serialize(serializer).unwrap();
}

#[test]
fn test_serialize_when_human_readable_false_with_empty_octets() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = std::io::Error;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn octets(&self) -> &[u8] {
            b""
        }

        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                // This block won't execute
                Ok(())
            } else {
                self.octets().serialize(serializer)
            }
        }
    }

    let test_value = TestStruct;
    let serializer = TestSerializer;

    test_value.serialize(serializer).unwrap();
}

