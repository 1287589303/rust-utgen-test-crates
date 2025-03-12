// Answer 0

#[test]
fn test_fmt_with_non_empty_bucket() {
    struct TestKey {
        value: i32,
    }

    impl fmt::Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestKey({})", self.value)
        }
    }

    struct TestValue {
        value: i32,
    }

    impl fmt::Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestValue({})", self.value)
        }
    }

    let bucket = Bucket {
        hash: HashValue::new(1),
        key: TestKey { value: 10 },
        value: TestValue { value: 20 },
    };

    let bucket_vec = vec![bucket];
    let mut drain = Drain {
        iter: bucket_vec.drain(..),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = drain.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_multiple_buckets() {
    struct TestKey {
        value: i32,
    }

    impl fmt::Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestKey({})", self.value)
        }
    }

    struct TestValue {
        value: i32,
    }

    impl fmt::Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestValue({})", self.value)
        }
    }

    let bucket1 = Bucket {
        hash: HashValue::new(2),
        key: TestKey { value: 1 },
        value: TestValue { value: 2 },
    };

    let bucket2 = Bucket {
        hash: HashValue::new(3),
        key: TestKey { value: 3 },
        value: TestValue { value: 4 },
    };

    let bucket_vec = vec![bucket1, bucket2];
    let mut drain = Drain {
        iter: bucket_vec.drain(..),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = drain.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_single_bucket() {
    struct TestKey {
        value: i32,
    }

    impl fmt::Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestKey({})", self.value)
        }
    }

    struct TestValue {
        value: i32,
    }

    impl fmt::Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestValue({})", self.value)
        }
    }

    let bucket = Bucket {
        hash: HashValue::new(4),
        key: TestKey { value: 5 },
        value: TestValue { value: 6 },
    };

    let bucket_vec = vec![bucket];
    let mut drain = Drain {
        iter: bucket_vec.drain(..),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = drain.fmt(&mut formatter);
}

