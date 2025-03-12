// Answer 0

#[test]
fn test_serialize_struct_success_start_fail_end() {
    struct TestSerializer;
    struct TestError;

    impl Error for TestError {
        fn custom<T: std::fmt::Display>(_: T) -> Self {
            TestError
        }
    }

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = TestError;

        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = TestStructSerializer;
        type SerializeStructVariant = ();

        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(TestStructSerializer)
        }

        fn is_human_readable(&self) -> bool {
            true
        }
    }

    struct TestStructSerializer;

    impl SerializeStruct for TestStructSerializer {
        type Ok = ();
        type Error = TestError;

        fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(TestError)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    struct RangeInclusive {
        start: i32,
        end: i32,
    }

    impl RangeInclusive {
        fn start(&self) -> &i32 {
            &self.start
        }

        fn end(&self) -> &i32 {
            &self.end
        }
    }

    let range = RangeInclusive { start: 1, end: 10 };
    let serializer = TestSerializer;
    let _ = range.serialize(serializer);
}

#[test]
fn test_serialize_struct_fail_start_success_end() {
    struct TestSerializer;
    struct TestError;

    impl Error for TestError {
        fn custom<T: std::fmt::Display>(_: T) -> Self {
            TestError
        }
    }

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = TestError;

        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = TestStructSerializer;
        type SerializeStructVariant = ();

        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(TestStructSerializer)
        }

        fn is_human_readable(&self) -> bool {
            true
        }
    }

    struct TestStructSerializer;

    impl SerializeStruct for TestStructSerializer {
        type Ok = ();
        type Error = TestError;

        fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(TestError)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    struct RangeInclusive {
        start: i32,
        end: i32,
    }

    impl RangeInclusive {
        fn start(&self) -> &i32 {
            &self.start
        }

        fn end(&self) -> &i32 {
            &self.end
        }
    }

    let range = RangeInclusive { start: 1, end: 10 };
    let serializer = TestSerializer;
    let _ = range.serialize(serializer);
}

