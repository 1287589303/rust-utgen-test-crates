// Answer 0

#[test]
fn test_put_with_exact_remaining() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
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
            let start = self.cursor;
            let end = self.data.len();
            UninitSlice::new(&mut self.data[start..end])
        }
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining_mut()
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
    let src = vec![1u8, 2u8, 3u8, 4u8, 5u8];

    buf.put(&src[..5]); // remaining_mut == 5, src.remaining() == 5
}

#[test]
fn test_put_with_partial_transfer() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
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
            let start = self.cursor;
            let end = self.data.len();
            UninitSlice::new(&mut self.data[start..end])
        }
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining_mut()
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

    let mut buf = TestBuf::new(10);
    let src = vec![1u8, 2u8, 3u8];

    buf.put(&src[..3]); // remaining_mut == 10, src.remaining() == 3
}

#[test]
#[should_panic]
fn test_put_with_not_enough_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
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
            let start = self.cursor;
            let end = self.data.len();
            UninitSlice::new(&mut self.data[start..end])
        }
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining_mut()
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

    let mut buf = TestBuf::new(2);
    let src = vec![1u8, 2u8, 3u8, 4u8]; // src.remaining() == 4 > remaining_mut == 2

    buf.put(&src[..]); // This should panic
}

