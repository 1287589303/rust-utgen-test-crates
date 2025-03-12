// Answer 0

#[test]
fn test_serialize_f64_nan() {
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

        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Float))
        }

        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_f64(f64::NAN);
}

#[test]
fn test_serialize_f64_positive_infinity() {
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

        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Float))
        }

        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_negative_infinity() {
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

        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Float))
        }

        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_f64(f64::NEG_INFINITY);
}

#[test]
fn test_serialize_f64_valid_finite() {
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

        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Float))
        }

        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_f64(1.0);
}

#[test]
fn test_serialize_f64_negative_finite() {
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

        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Float))
        }

        fn bad_type(_: Unsupported) -> Self::Error {
            Error
        }
    }

    let serializer = TestSerializer;
    let _ = serializer.serialize_f64(-1.0);
}

