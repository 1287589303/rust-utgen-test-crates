// Answer 0

#[test]
#[should_panic]
fn test_put_uint_le_panics_when_nbytes_greater_than_8() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBuf {
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
            let len = self.remaining_mut();
            &mut self.data[self.position..self.position + len] as *mut _ as *mut UninitSlice
        }

        fn put_slice(&mut self, slice: &[u8]) {
            let len = slice.len();
            self.data.extend_from_slice(slice);
            unsafe { self.advance_mut(len) };
        }

        fn put_uint_le(&mut self, n: u64, nbytes: usize) {
            let slice = n.to_le_bytes();
            let slice = match slice.get(..nbytes) {
                Some(slice) => slice,
                None => panic_does_not_fit(nbytes, slice.len()),
            };
            self.put_slice(slice);
        }
    }

    let mut buf = TestBuf {
        data: Vec::new(),
        position: 0,
    };

    // This should panic as nbytes is greater than 8
    buf.put_uint_le(0x0102030405060708, 9);
}

