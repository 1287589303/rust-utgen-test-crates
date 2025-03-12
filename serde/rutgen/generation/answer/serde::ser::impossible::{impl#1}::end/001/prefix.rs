// Answer 0

#[test]
fn test_end_with_valid_ok_and_error() {
    struct ValidOk;
    struct ValidError;

    impl ser::Error for ValidError {}

    let instance = Impossible::<ValidOk, ValidError> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = instance.end();
}

#[test]
#[should_panic]
fn test_end_with_uninitialised_void() {
    struct TestOk;
    struct TestError;

    impl ser::Error for TestError {}

    let instance = Impossible::<TestOk, TestError> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = instance.end();
}

#[test]
fn test_end_with_minimal_ok_and_error() {
    struct MinimalOk;
    struct MinimalError;

    impl ser::Error for MinimalError {}

    let instance = Impossible::<MinimalOk, MinimalError> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = instance.end();
}

#[test]
fn test_end_with_other_custom_error() {
    struct CustomOk;
    struct CustomError;

    impl ser::Error for CustomError {}

    let instance = Impossible::<CustomOk, CustomError> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = instance.end();
}

#[test]
fn test_end_with_edge_case_types() {
    struct EdgeOk;
    struct EdgeError;

    impl ser::Error for EdgeError {}

    let instance = Impossible::<EdgeOk, EdgeError> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let _ = instance.end();
}

