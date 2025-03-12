// Answer 0

#[test]
fn test_collect_seq_empty_iterator() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = std::marker::PhantomData<()>;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(std::marker::PhantomData)
        }
        
        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    let serializer = TestSerializer;
    let result: Result<(), _> = serializer.collect_seq(vec![].into_iter());
}

#[test]
fn test_collect_seq_custom_type_without_serialize() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = std::marker::PhantomData<()>;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(std::marker::PhantomData)
        }
        
        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    struct NonSerializable;

    let serializer = TestSerializer;
    let result: Result<(), _> = serializer.collect_seq(vec![NonSerializable].into_iter());
}

#[test]
fn test_collect_seq_failing_on_known_failure() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = std::marker::PhantomData<()>;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(std::marker::PhantomData)
        }
        
        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(std::marker::PhantomData)
        }
    }

    let serializer = TestSerializer;
    let result: Result<(), _> = serializer.collect_seq(vec![FailingSerialize].into_iter());
}

