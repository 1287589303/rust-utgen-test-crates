// Answer 0

#[test]
fn test_cause_left_non_empty() {
    struct DummyError1;
    impl fmt::Debug for DummyError1 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "DummyError1")
        }
    }
    impl Error for DummyError1 {}

    let left_error = DummyError1;
    let either = Either::Left(left_error);
    let _result = either.cause();
}

#[test]
fn test_cause_right_non_empty() {
    struct DummyError2;
    impl fmt::Debug for DummyError2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "DummyError2")
        }
    }
    impl Error for DummyError2 {}

    let right_error = DummyError2;
    let either = Either::Right(right_error);
    let _result = either.cause();
}

#[test]
fn test_cause_left_empty() {
    struct NoError;
    impl fmt::Debug for NoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "NoError")
        }
    }
    impl Error for NoError {}

    let left_error = NoError;
    let either = Either::Left(left_error);
    let _result = either.cause();
}

#[test]
fn test_cause_right_empty() {
    struct NoError;
    impl fmt::Debug for NoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "NoError")
        }
    }
    impl Error for NoError {}

    let right_error = NoError;
    let either = Either::Right(right_error);
    let _result = either.cause();
}

