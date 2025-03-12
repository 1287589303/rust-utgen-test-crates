// Answer 0

#[test]
fn test_put_i16_valid() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { data: Vec::with_capacity(capacity), capacity }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Mock implementation
            &mut UninitSlice::from_slice(&mut self.data[self.data.len()..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let dst = self.chunk_mut();
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }

        fn put_i16(&mut self, n: i16) {
            self.put_slice(&n.to_be_bytes());
        }
    }

    let mut buf = TestBuf::new(4);
    buf.put_i16(0x0809);
}

#[test]
#[should_panic]
fn test_put_i16_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { data: Vec::with_capacity(capacity), capacity }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::from_slice(&mut self.data[self.data.len()..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let dst = self.chunk_mut();
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }

        fn put_i16(&mut self, n: i16) {
            self.put_slice(&n.to_be_bytes());
        }
    }

    let mut buf = TestBuf::new(1);
    buf.put_i16(0x0809);
}

#[test]
fn test_put_i16_boundary_case() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { data: Vec::with_capacity(capacity), capacity }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::from_slice(&mut self.data[self.data.len()..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let dst = self.chunk_mut();
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }

        fn put_i16(&mut self, n: i16) {
            self.put_slice(&n.to_be_bytes());
        }
    }

    let mut buf = TestBuf::new(2);
    buf.put_i16(i16::MAX);
}

