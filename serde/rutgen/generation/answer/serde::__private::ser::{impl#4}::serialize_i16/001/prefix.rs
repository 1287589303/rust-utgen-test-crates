// Answer 0

#[test]
fn test_serialize_i16_negative_boundary() {
    struct TestSerializer;
    
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = Impossible<Self::Ok, Self::Error>;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Integer))
        }
        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_i16(-32768);
}

#[test]
fn test_serialize_i16_negative() {
    struct TestSerializer;
    
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = Impossible<Self::Ok, Self::Error>;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Integer))
        }
        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_i16(-1);
}

#[test]
fn test_serialize_i16_zero() {
    struct TestSerializer;
    
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = Impossible<Self::Ok, Self::Error>;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Integer))
        }
        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_positive() {
    struct TestSerializer;
    
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = Impossible<Self::Ok, Self::Error>;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Integer))
        }
        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_i16(1);
}

#[test]
fn test_serialize_i16_positive_boundary() {
    struct TestSerializer;
    
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = Impossible<Self::Ok, Self::Error>;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Integer))
        }
        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_i16(32767);
}

