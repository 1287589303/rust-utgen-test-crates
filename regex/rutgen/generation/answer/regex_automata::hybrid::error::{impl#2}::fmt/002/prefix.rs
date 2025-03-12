// Answer 0

#[test]
fn test_insufficient_state_id_capacity() {
    struct TestLazyStateIDError {
        attempted: u64,
    }

    impl LazyStateIDError {
        fn new(attempted: u64) -> Self {
            LazyStateIDError { attempted }
        }
    }

    let err = TestLazyStateIDError { attempted: 0 };
    let build_error = BuildError {
        kind: BuildErrorKind::InsufficientStateIDCapacity { err: err.clone() },
    };
    let mut formatter = core::fmt::Formatter::new();

    build_error.fmt(&mut formatter);
}

#[test]
fn test_insufficient_state_id_capacity_boundary() {
    struct TestLazyStateIDError {
        attempted: u64,
    }

    impl LazyStateIDError {
        fn new(attempted: u64) -> Self {
            LazyStateIDError { attempted }
        }
    }

    let err = TestLazyStateIDError { attempted: LazyStateID::MAX };
    let build_error = BuildError {
        kind: BuildErrorKind::InsufficientStateIDCapacity { err: err.clone() },
    };
    let mut formatter = core::fmt::Formatter::new();

    build_error.fmt(&mut formatter);
}

#[test]
fn test_insufficient_state_id_capacity_high_value() {
    struct TestLazyStateIDError {
        attempted: u64,
    }

    impl LazyStateIDError {
        fn new(attempted: u64) -> Self {
            LazyStateIDError { attempted }
        }
    }

    let err = TestLazyStateIDError { attempted: LazyStateID::MAX - 1 };
    let build_error = BuildError {
        kind: BuildErrorKind::InsufficientStateIDCapacity { err: err.clone() },
    };
    let mut formatter = core::fmt::Formatter::new();

    build_error.fmt(&mut formatter);
}

