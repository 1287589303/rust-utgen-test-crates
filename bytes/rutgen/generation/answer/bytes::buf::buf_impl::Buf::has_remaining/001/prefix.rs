// Answer 0

#[test]
fn test_has_remaining_with_non_empty_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.position += 1;
            byte
        }
    }

    let mut buf = TestBuf::new(vec![1, 2, 3]);
    assert!(buf.has_remaining());
    buf.get_u8();
    assert!(buf.has_remaining());
}

#[test]
fn test_has_remaining_with_single_byte_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.position += 1;
            byte
        }
    }

    let mut buf = TestBuf::new(vec![5]);
    assert!(buf.has_remaining());
    buf.get_u8();
    assert!(!buf.has_remaining());
}

#[test]
fn test_has_remaining_with_empty_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
    }

    let buf = TestBuf::new(vec![]);
    assert!(!buf.has_remaining());
}

