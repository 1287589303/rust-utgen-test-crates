// Answer 0

#[test]
fn test_put_i128_le_with_minimum_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
                position: 0,
            }
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.position..self.position + remaining]
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let dst = self.chunk_mut();
            let cnt = usize::min(src.len(), dst.len());
            dst[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(16); // exact capacity for 16 bytes
    buf.put_slice(&0x01020304050607080910111213141516_i128.to_le_bytes());
}

#[test]
fn test_put_i128_le_with_large_value() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
                position: 0,
            }
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.position..self.position + remaining]
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let dst = self.chunk_mut();
            let cnt = usize::min(src.len(), dst.len());
            dst[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(16); // exact capacity for 16 bytes
    buf.put_slice(&i128::MAX.to_le_bytes());
}

#[test]
fn test_put_i128_le_with_small_positive_value() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
                position: 0,
            }
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.position..self.position + remaining]
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let dst = self.chunk_mut();
            let cnt = usize::min(src.len(), dst.len());
            dst[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(16); // enough capacity
    buf.put_slice(&1_i128.to_le_bytes());
}

#[test]
fn test_put_i128_le_with_small_negative_value() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
                position: 0,
            }
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.position..self.position + remaining]
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let dst = self.chunk_mut();
            let cnt = usize::min(src.len(), dst.len());
            dst[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(16); // enough capacity
    buf.put_slice(&(-1_i128).to_le_bytes());
}

#[test]
#[should_panic]
fn test_put_i128_le_with_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: vec![0; capacity],
                position: 0,
            }
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.position..self.position + remaining]
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough space");
            }
            let dst = self.chunk_mut();
            let cnt = usize::min(src.len(), dst.len());
            dst[..cnt].copy_from_slice(&src[..cnt]);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(15); // not enough capacity
    buf.put_slice(&0x01020304050607080910111213141516_i128.to_le_bytes());
}

