// Answer 0

#[test]
#[should_panic]
fn test_put_int_le_nbytes_greater_than_8() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            TestBuf {
                data: vec![0; size],
                pos: 0,
            }
        }
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Implementation of this method is not required for the test.
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            // Implementation of this method is not required for the test.
            unimplemented!()
        }
        fn put_int_le(&mut self, n: i64, nbytes: usize) {
            let slice = n.to_le_bytes();
            let slice = match slice.get(..nbytes) {
                Some(slice) => slice,
                None => panic_does_not_fit(nbytes, slice.len()),
            };
            self.put_slice(slice);
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_int_le(0x0504010203040506, 9);
}

#[test]
#[should_panic]
fn test_put_int_le_nbytes_negative() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            TestBuf {
                data: vec![0; size],
                pos: 0,
            }
        }
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Implementation of this method is not required for the test.
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            // Implementation of this method is not required for the test.
            unimplemented!()
        }
        fn put_int_le(&mut self, n: i64, nbytes: usize) {
            let slice = n.to_le_bytes();
            let slice = match slice.get(..nbytes) {
                Some(slice) => slice,
                None => panic_does_not_fit(nbytes, slice.len()),
            };
            self.put_slice(slice);
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_int_le(0x0504010203040506, -1);
}

