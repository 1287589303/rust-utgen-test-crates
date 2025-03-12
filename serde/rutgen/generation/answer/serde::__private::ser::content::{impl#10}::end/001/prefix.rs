// Answer 0

#[test]
fn test_end_with_non_empty_name_and_variant() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl serde::ser::Error for DummyError {
        fn custom<T>(_msg: T) -> Self {
            DummyError
        }
    }

    let serializer = SerializeTupleVariant::<DummyError> {
        name: "test_variant",
        variant_index: 1,
        variant: "test",
        fields: vec![Content::U32(42)],
        error: std::marker::PhantomData,
    };
    let _result = serializer.end();
}

#[test]
fn test_end_with_empty_fields() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl serde::ser::Error for DummyError {
        fn custom<T>(_msg: T) -> Self {
            DummyError
        }
    }

    let serializer = SerializeTupleVariant::<DummyError> {
        name: "empty_fields",
        variant_index: 0,
        variant: "empty_variant",
        fields: Vec::new(),
        error: std::marker::PhantomData,
    };
    let _result = serializer.end();
}

#[test]
fn test_end_with_non_negative_variant_index() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl serde::ser::Error for DummyError {
        fn custom<T>(_msg: T) -> Self {
            DummyError
        }
    }

    let serializer = SerializeTupleVariant::<DummyError> {
        name: "valid_variant",
        variant_index: 0,
        variant: "variant_name",
        fields: vec![Content::I32(-1)],
        error: std::marker::PhantomData,
    };
    let _result = serializer.end();
}

#[test]
fn test_end_with_large_variant_index() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl serde::ser::Error for DummyError {
        fn custom<T>(_msg: T) -> Self {
            DummyError
        }
    }

    let serializer = SerializeTupleVariant::<DummyError> {
        name: "large_index",
        variant_index: 1_000_000,
        variant: "big_variant",
        fields: vec![Content::F64(3.14)],
        error: std::marker::PhantomData,
    };
    let _result = serializer.end();
}

