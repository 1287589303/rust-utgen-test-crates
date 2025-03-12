// Answer 0

#[test]
fn test_end_with_unit_ok() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let impossible: Impossible<(), DummyError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = impossible.end();
}

#[test]
fn test_end_with_string_ok() {
    struct StringError;
    impl ser::Error for StringError {}

    let impossible: Impossible<String, StringError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = impossible.end();
}

#[test]
fn test_end_with_vec_ok() {
    struct VecError;
    impl ser::Error for VecError {}

    let impossible: Impossible<Vec<i32>, VecError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = impossible.end();
}

#[test]
fn test_end_with_custom_struct_ok() {
    struct CustomStruct;
    struct CustomError;
    impl ser::Error for CustomError {}

    let impossible: Impossible<CustomStruct, CustomError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = impossible.end();
}

#[test]
fn test_end_with_no_type_ok() {
    struct NoTypeError;
    impl ser::Error for NoTypeError {}

    let impossible: Impossible<_, NoTypeError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = impossible.end();
}

