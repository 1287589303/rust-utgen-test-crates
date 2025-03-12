// Answer 0

#[test]
fn test_serialize_u8_zero() {
    struct TestSerializer;
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Self::Error>;
        type SerializeTuple = Impossible<(), Self::Error>;
        type SerializeTupleStruct = Impossible<(), Self::Error>;
        type SerializeTupleVariant = Impossible<(), Self::Error>;
        type SerializeMap = Impossible<(), Self::Error>;
        type SerializeStruct = Impossible<(), Self::Error>;

        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
            Err(Error)
        }

        // Other functions in the trait would need to be implemented as well.
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u8(0);
    // No assertion
}

#[test]
fn test_serialize_u8_one() {
    struct TestSerializer;
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Self::Error>;
        type SerializeTuple = Impossible<(), Self::Error>;
        type SerializeTupleStruct = Impossible<(), Self::Error>;
        type SerializeTupleVariant = Impossible<(), Self::Error>;
        type SerializeMap = Impossible<(), Self::Error>;
        type SerializeStruct = Impossible<(), Self::Error>;

        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
            Err(Error)
        }

        // Other functions in the trait would need to be implemented as well.
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u8(1);
    // No assertion
}

#[test]
fn test_serialize_u8_max() {
    struct TestSerializer;
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Self::Error>;
        type SerializeTuple = Impossible<(), Self::Error>;
        type SerializeTupleStruct = Impossible<(), Self::Error>;
        type SerializeTupleVariant = Impossible<(), Self::Error>;
        type SerializeMap = Impossible<(), Self::Error>;
        type SerializeStruct = Impossible<(), Self::Error>;

        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
            Err(Error)
        }

        // Other functions in the trait would need to be implemented as well.
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u8(255);
    // No assertion
}

