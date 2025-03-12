// Answer 0

#[test]
fn test_set_limit_to_zero() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl MockBuf {
        fn take(limit: usize) -> Take<MockBuf> {
            Take {
                inner: MockBuf {
                    data: b"example data".to_vec(),
                    position: 0,
                },
                limit,
            }
        }
    }

    let mut buf = MockBuf::take(5);
    buf.set_limit(0);
}

#[test]
fn test_set_limit_to_one() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl MockBuf {
        fn take(limit: usize) -> Take<MockBuf> {
            Take {
                inner: MockBuf {
                    data: b"example data".to_vec(),
                    position: 0,
                },
                limit,
            }
        }
    }

    let mut buf = MockBuf::take(5);
    buf.set_limit(1);
}

#[test]
fn test_set_limit_to_max_usize() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl MockBuf {
        fn take(limit: usize) -> Take<MockBuf> {
            Take {
                inner: MockBuf {
                    data: b"example data".to_vec(),
                    position: 0,
                },
                limit,
            }
        }
    }

    let mut buf = MockBuf::take(5);
    buf.set_limit(usize::MAX);
}

