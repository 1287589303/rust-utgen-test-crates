// Answer 0

#[test]
fn test_put_i32_with_exact_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buffer: Vec::with_capacity(capacity),
                capacity,
            }
        }
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.capacity - self.buffer.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.buffer.set_len(self.buffer.len() + cnt);
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume UninitSlice can be represented as a slice for this purpose
            let len = self.remaining_mut();
            // Safety: we have ensured there's enough capacity
            &mut *(self.buffer.as_mut_slice().get_unchecked_mut(..len) as *mut _)
        }

        fn put_slice(&mut self, src: &[u8]) {
            for &byte in src {
                self.buffer.push(byte);
            }
        }

        fn put_i32(&mut self, n: i32) {
            self.put_slice(&n.to_be_bytes());
        }
    }

    let mut buf = TestBuf::new(4);
    buf.put_i32(0x0809A0A1);
}

#[test]
#[should_panic]
fn test_put_i32_with_insufficient_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buffer: Vec::with_capacity(capacity),
                capacity,
            }
        }
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.capacity - self.buffer.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.buffer.set_len(self.buffer.len() + cnt);
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume UninitSlice can be represented as a slice for this purpose
            let len = self.remaining_mut();
            // Safety: we have ensured there's enough capacity
            &mut *(self.buffer.as_mut_slice().get_unchecked_mut(..len) as *mut _)
        }

        fn put_slice(&mut self, src: &[u8]) {
            for &byte in src {
                self.buffer.push(byte);
            }
        }

        fn put_i32(&mut self, n: i32) {
            self.put_slice(&n.to_be_bytes());
        }
    }

    let mut buf = TestBuf::new(3);
    buf.put_i32(0x0809A0A1);
}

#[test]
fn test_put_i32_with_extra_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buffer: Vec::with_capacity(capacity),
                capacity,
            }
        }
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.capacity - self.buffer.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.buffer.set_len(self.buffer.len() + cnt);
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume UninitSlice can be represented as a slice for this purpose
            let len = self.remaining_mut();
            // Safety: we have ensured there's enough capacity
            &mut *(self.buffer.as_mut_slice().get_unchecked_mut(..len) as *mut _)
        }

        fn put_slice(&mut self, src: &[u8]) {
            for &byte in src {
                self.buffer.push(byte);
            }
        }

        fn put_i32(&mut self, n: i32) {
            self.put_slice(&n.to_be_bytes());
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_i32(-1);
    buf.put_i32(2_147_483_647);
}

