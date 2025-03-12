// Answer 0

#[test]
fn test_serialize_newtype_struct_err() {
    struct ErrorStub;

    impl ser::Error for ErrorStub {
        fn custom<T>(_: T) -> Self {
            ErrorStub
        }
    }

    let serializer = ContentSerializer::<ErrorStub> {
        error: PhantomData,
    };

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _: S) -> Result<Content, S::Error>
        where
            S: Serializer,
        {
            Err(ErrorStub::custom("serialization failed"))
        }
    }

    let value = FailingSerialize;
    let result = serializer.serialize_newtype_struct("test", &value);
}

