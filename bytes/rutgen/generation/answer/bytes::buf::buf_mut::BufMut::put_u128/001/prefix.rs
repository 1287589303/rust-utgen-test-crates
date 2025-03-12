// Answer 0

#[test]
fn test_put_u128_with_minimum_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                position: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::from(&mut self.data[self.position..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough remaining capacity");
            }
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.chunk_mut()[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(16);
    buf.put_u128(0x01020304050607080910111213141516);
}

#[test]
fn test_put_u128_with_excess_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                position: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::from(&mut self.data[self.position..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough remaining capacity");
            }
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.chunk_mut()[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(32);
    buf.put_u128(0x01020304050607080910111213141516);
}

#[test]
#[should_panic]
fn test_put_u128_with_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                position: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::from(&mut self.data[self.position..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough remaining capacity");
            }
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.chunk_mut()[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(15);
    buf.put_u128(0x01020304050607080910111213141516);
}

