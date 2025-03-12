// Answer 0

#[test]
#[should_panic]
fn test_put_uint_panics_when_nbytes_greater_than_8() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assuming UninitSlice has some implementation to get a mutable slice
            let slice = &mut self.data[self.position..];
            unsafe { UninitSlice::from_mut(slice) }
        }
    }

    let mut buf = TestBuf { data: vec![0; 10], position: 0 };
    buf.put_uint(0x01, 9);
}

#[test]
fn test_put_uint_successful_write() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let slice = &mut self.data[self.position..];
            unsafe { UninitSlice::from_mut(slice) }
        }
    }

    let mut buf = TestBuf { data: vec![0; 10], position: 0 };
    buf.put_uint(0x01020304, 4); // nbytes = 4
    // further verification of contents can occur here if needed
}

#[test]
#[should_panic]
fn test_put_uint_panics_with_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let slice = &mut self.data[self.position..];
            unsafe { UninitSlice::from_mut(slice) }
        }
    }

    let mut buf = TestBuf { data: vec![0; 5], position: 0 };
    buf.put_uint(0x01, 6); // Will panic as nbytes > remaining capacity
}

