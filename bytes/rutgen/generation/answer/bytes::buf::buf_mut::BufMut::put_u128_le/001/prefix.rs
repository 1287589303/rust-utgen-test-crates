// Answer 0

#[test]
fn test_put_u128_le_valid() {
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
            // Assume uninitialized slice representation for testing purposes
            &mut self.data[self.position..].into()
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let mut src = src;
            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());
                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buf = TestBuf::new(32);
    buf.put_u128_le(0x01020304050607080910111213141516);
}

#[test]
fn test_put_u128_le_boundary() {
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
            &mut self.data[self.position..].into()
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let mut src = src;
            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());
                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buf = TestBuf::new(16);
    buf.put_u128_le(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
}

