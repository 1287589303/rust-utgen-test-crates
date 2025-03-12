// Answer 0

#[test]
fn test_debug_slice_empty() {
    struct TestType;
    impl fmt::Debug for TestType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestType")
        }
    }

    let slice: Slice<TestType> = Slice { entries: [] };
    let _ = fmt::Debug::fmt(&slice, &mut fmt::Formatter::new());
}

#[test]
fn test_debug_slice_single_entry() {
    struct TestType {
        value: i32,
    }
    impl fmt::Debug for TestType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestType({})", self.value)
        }
    }

    let bucket = Bucket {
        hash: HashValue::new(),
        key: TestType { value: 42 },
        value: TestType { value: 42 },
    };
    let slice = Slice {
        entries: [bucket],
    };
    let _ = fmt::Debug::fmt(&slice, &mut fmt::Formatter::new());
}

#[test]
fn test_debug_slice_multiple_entries() {
    struct TestType {
        name: String,
        value: f64,
    }
    impl fmt::Debug for TestType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestType({}, {})", self.name, self.value)
        }
    }

    let buckets = [
        Bucket {
            hash: HashValue::new(),
            key: TestType {
                name: "A".to_string(),
                value: 3.14,
            },
            value: TestType {
                name: "A".to_string(),
                value: 3.14,
            },
        },
        Bucket {
            hash: HashValue::new(),
            key: TestType {
                name: "B".to_string(),
                value: 2.71,
            },
            value: TestType {
                name: "B".to_string(),
                value: 2.71,
            },
        },
    ];

    let slice = Slice { entries: buckets };
    let _ = fmt::Debug::fmt(&slice, &mut fmt::Formatter::new());
}

