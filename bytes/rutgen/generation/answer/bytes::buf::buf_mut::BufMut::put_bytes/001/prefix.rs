// Answer 0

#[test]
fn test_put_bytes_panic_case1() {
    struct TestBuf {
        data: [u8; 5],
        position: usize,
    }
    
    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            UninitSlice::new(&mut self.data[self.position..])
        }
        
        fn put_u8(&mut self, _n: u8) {}
    }
    
    let mut buf = TestBuf { data: [0; 5], position: 0 };
    buf.put_bytes(b'a', 6); // Panic expected
}

#[test]
fn test_put_bytes_panic_case2() {
    struct TestBuf {
        data: [u8; 10],
        position: usize,
    }
    
    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            UninitSlice::new(&mut self.data[self.position..])
        }

        fn put_u8(&mut self, _n: u8) {}
    }
    
    let mut buf = TestBuf { data: [0; 10], position: 5 };
    buf.put_bytes(b'b', 6); // Panic expected
}

#[test]
fn test_put_bytes_panic_case3() {
    struct TestBuf {
        data: [u8; 15],
        position: usize,
    }
    
    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            UninitSlice::new(&mut self.data[self.position..])
        }

        fn put_u8(&mut self, _n: u8) {}
    }
    
    let mut buf = TestBuf { data: [0; 15], position: 10 };
    buf.put_bytes(b'c', 6); // Panic expected
}

