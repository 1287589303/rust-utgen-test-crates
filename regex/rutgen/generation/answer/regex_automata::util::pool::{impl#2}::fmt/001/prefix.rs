// Answer 0

#[test]
fn test_fmt_empty_stack() {
    use std::sync::Mutex;
    use std::fmt;

    struct TestType;

    impl fmt::Debug for TestType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestType")
        }
    }

    fn create_test_type() -> TestType {
        TestType
    }

    let pool = Pool::<TestType>::new(Box::new(create_test_type));
    let formatter = fmt::Formatter::new();
    pool.fmt(&formatter);
}

#[test]
fn test_fmt_single_element() {
    use std::sync::Mutex;
    use std::fmt;

    struct TestType;

    impl fmt::Debug for TestType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestType")
        }
    }

    fn create_test_type() -> TestType {
        TestType
    }

    let mut pool = Pool::<TestType>::new(Box::new(create_test_type));
    pool.push(TestType);
    let formatter = fmt::Formatter::new();
    pool.fmt(&formatter);
}

#[test]
fn test_fmt_large_stack() {
    use std::sync::Mutex;
    use std::fmt;

    struct TestType;

    impl fmt::Debug for TestType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestType")
        }
    }

    fn create_test_type() -> TestType {
        TestType
    }

    let mut pool = Pool::<TestType>::new(Box::new(create_test_type));
    for _ in 0..1_000 {
        pool.push(TestType);
    }
    let formatter = fmt::Formatter::new();
    pool.fmt(&formatter);
}

#[test]
fn test_fmt_full_stack() {
    use std::sync::Mutex;
    use std::fmt;

    struct TestType;

    impl fmt::Debug for TestType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestType")
        }
    }

    fn create_test_type() -> TestType {
        TestType
    }

    let mut pool = Pool::<TestType>::new(Box::new(create_test_type));
    for _ in 0..10 {
        pool.push(TestType);
    }
    let formatter = fmt::Formatter::new();
    pool.fmt(&formatter);
}

