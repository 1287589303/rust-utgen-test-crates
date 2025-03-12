// Answer 0

#[test]
fn test_get_i8_valid_value() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    let mut buf = TestBuf {
        data: vec![128], // Valid u8 value that converts to i8
        pos: 0,
    };
    let value = buf.get_i8();
}

#[test]
fn test_get_i8_max_edge_value() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    let mut buf = TestBuf {
        data: vec![255], // Maximum u8 value
        pos: 0,
    };
    let value = buf.get_i8();
}

#[test]
#[should_panic]
fn test_get_i8_no_remaining_data() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    let mut buf = TestBuf {
        data: vec![], // No data present
        pos: 0,
    };
    let value = buf.get_i8();
}

