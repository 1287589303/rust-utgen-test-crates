// Answer 0

#[test]
fn test_try_get_i8_success() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
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

    let mut buf = TestBuf {
        data: vec![0x08],
        position: 0,
    };
    let result = buf.try_get_i8();
}

