// Answer 0

#[test]
fn test_put_with_exact_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                cursor: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assuming size fits the remaining mut
            UninitSlice::new(&mut self.data[self.cursor..])
        }
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.cursor
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.cursor..]
        }

        fn advance(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let mut buf = TestBuf::new(5);
    let src = vec![1, 2, 3, 4, 5]; // `src.remaining()` is 5
    buf.put(&src[..]); // This should work without panic
}

#[test]
fn test_put_with_empty_src() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                cursor: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            UninitSlice::new(&mut self.data[self.cursor..])
        }
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            0 // No remaining bytes in the source
        }

        fn chunk(&self) -> &[u8] {
            &[] // No chunk in the source
        }

        fn advance(&mut self, _cnt: usize) {}

        fn has_remaining(&self) -> bool {
            false
        }
    }

    let mut buf = TestBuf::new(10);
    buf.put(vec![]); // Should handle a case where source has no remaining bytes without panic
}

