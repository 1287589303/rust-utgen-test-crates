// Answer 0

#[test]
fn test_put_uint_with_various_nbytes() {
    struct TestBuf {
        buffer: Vec<u8>,
        pos: usize,
    }
    
    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buffer: vec![0; capacity],
                pos: 0,
            }
        }
        
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.pos
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // This would normally return a mutable slice; for simplicity, using a raw pointer
            // directly. This will not actually be safe as is, so this is for illustrative purposes.
            let start = self.pos;
            let end = self.pos + self.remaining_mut();
            &mut self.buffer[start..end]
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.chunk_mut()[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }
    
    let mut buf = TestBuf::new(8);
    let values: [(u64, usize); 4] = [
        (0x010203, 3),
        (0xFFFFFFFFFFFFFFFF, 8),
        (0x12345678, 4),
        (0x00, 1),
    ];

    for (n, nbytes) in values.iter() {
        buf.put_uint(*n, *nbytes);
    }
}

#[test]
#[should_panic]
fn test_put_uint_panic_for_nbytes_greater_than_size_of_n() {
    struct TestBuf {
        buffer: Vec<u8>,
        pos: usize,
    }
    
    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buffer: vec![0; capacity],
                pos: 0,
            }
        }
        
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.pos
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let start = self.pos;
            let end = self.pos + self.remaining_mut();
            &mut self.buffer[start..end]
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.chunk_mut()[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }
    
    let mut buf = TestBuf::new(8);
    buf.put_uint(0x01, 9);
}

