// Answer 0

#[test]
fn test_put_u8_valid() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough remaining capacity");
            }
            self.data.extend_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    impl BufMut for TestBuf {
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put(&mut self, src: impl super::Buf) {
            unimplemented!()
        }

        fn put_bytes(&mut self, _val: u8, _cnt: usize) {
            unimplemented!()
        }
    }

    let mut buf = TestBuf::new(5);
    buf.put_u8(0); // Minimum valid input
    buf.put_u8(255); // Maximum valid input
    buf.put_u8(128); // Mid-range input
}

#[test]
#[should_panic]
fn test_put_u8_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough remaining capacity");
            }
            self.data.extend_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    impl BufMut for TestBuf {
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put(&mut self, src: impl super::Buf) {
            unimplemented!()
        }

        fn put_bytes(&mut self, _val: u8, _cnt: usize) {
            unimplemented!()
        }
    }

    // Initialize `TestBuf` with a capacity of 1, so it can't accept more than one byte
    let mut buf = TestBuf::new(1);
    buf.put_u8(0); // Fill the buffer to its capacity
    buf.put_u8(1); // Attempt to add another byte, which should panic
}

