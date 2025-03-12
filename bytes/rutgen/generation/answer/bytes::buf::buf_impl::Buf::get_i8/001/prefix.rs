// Answer 0

#[test]
#[should_panic]
fn test_get_i8_panics_when_empty() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            TestBuf { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let mut buf = TestBuf::new(&[]);
    buf.get_i8();
}

#[test]
#[should_panic]
fn test_get_i8_panics_when_one_byte_buffer_is_consumed() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            TestBuf { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let mut buf = TestBuf::new(&[1]);
    buf.advance(1);
    buf.get_i8();
}

