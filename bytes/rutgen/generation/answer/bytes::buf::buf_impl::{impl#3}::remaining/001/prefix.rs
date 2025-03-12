// Answer 0

#[test]
fn test_remaining_zero_length() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf { data: vec![], position: 0 };
    let result = buf.remaining();
}

#[test]
fn test_remaining_non_zero_length_at_start() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf { data: vec![1, 2, 3], position: 0 };
    let result = buf.remaining();
}

#[test]
fn test_remaining_non_zero_length_at_mid() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf { data: vec![1, 2, 3], position: 1 };
    let result = buf.remaining();
}

#[test]
fn test_remaining_non_zero_length_at_end() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf { data: vec![1, 2, 3], position: 3 };
    let result = buf.remaining();
}

#[test]
fn test_remaining_position_exceeds_length() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf { data: vec![1, 2], position: 3 };
    let result = buf.remaining();
}

