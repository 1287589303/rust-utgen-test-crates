// Answer 0

#[test]
fn test_put_u64_ne_valid_endian() {
    struct BufMutImpl {
        buf: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.buf.len() - self.position
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::from(&mut self.buf[self.position..])
        }
    }

    let mut buf = BufMutImpl { buf: vec![0; 16], position: 0 };
    buf.put_u64_ne(0x0102030405060708);
}

#[test]
#[should_panic]
fn test_put_u64_ne_not_enough_capacity() {
    struct BufMutImpl {
        buf: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            0 // Insufficient capacity
        }
        
        unsafe fn advance_mut(&mut self, _cnt: usize) {
            // Do nothing
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Will not be used as it should panic
            &mut UninitSlice::from(&mut [])
        }
    }

    let mut buf = BufMutImpl { buf: vec![], position: 0 };
    buf.put_u64_ne(0x0102030405060708);
}

