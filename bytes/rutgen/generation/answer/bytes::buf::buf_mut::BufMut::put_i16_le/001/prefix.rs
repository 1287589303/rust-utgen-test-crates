// Answer 0

#[test]
fn test_put_i16_le_valid() {
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
            let start = self.position;
            let range = if len >= 2 { start..start + 2 } else { start..start };
            &mut (self.data[range].as_mut() as *mut _ as *mut UninitSlice)
        }
        
        fn put<T: super::Buf>(&mut self, _src: T) {}
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}
        fn put_u8(&mut self, _n: u8) {}
        fn put_i8(&mut self, _n: i8) {}
        fn put_u16(&mut self, _n: u16) {}
        fn put_u32(&mut self, _n: u32) {}
        fn put_i32(&mut self, _n: i32) {}
        fn put_u64(&mut self, _n: u64) {}
        fn put_i64(&mut self, _n: i64) {}
        fn put_u128(&mut self, _n: u128) {}
        fn put_i128(&mut self, _n: i128) {}
        fn put_uint(&mut self, _n: u64, _nbytes: usize) {}
        fn put_int(&mut self, _n: i64, _nbytes: usize) {}
        fn put_f32(&mut self, _n: f32) {}
        fn put_f64(&mut self, _n: f64) {}
    }

    let mut buf = TestBuf {
        data: vec![0; 10],
        position: 0,
    };

    buf.put_i16_le(1);
    buf.put_i16_le(-1);
    buf.put_i16_le(32767);
    buf.put_i16_le(-32768);
}

#[test]
#[should_panic]
fn test_put_i16_le_insufficient_capacity() {
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
            let start = self.position;
            let range = if len >= 2 { start..start + 2 } else { start..start };
            &mut (self.data[range].as_mut() as *mut _ as *mut UninitSlice)
        }
        
        fn put<T: super::Buf>(&mut self, _src: T) {}
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}
        fn put_u8(&mut self, _n: u8) {}
        fn put_i8(&mut self, _n: i8) {}
        fn put_u16(&mut self, _n: u16) {}
        fn put_u32(&mut self, _n: u32) {}
        fn put_i32(&mut self, _n: i32) {}
        fn put_u64(&mut self, _n: u64) {}
        fn put_i64(&mut self, _n: i64) {}
        fn put_u128(&mut self, _n: u128) {}
        fn put_i128(&mut self, _n: i128) {}
        fn put_uint(&mut self, _n: u64, _nbytes: usize) {}
        fn put_int(&mut self, _n: i64, _nbytes: usize) {}
        fn put_f32(&mut self, _n: f32) {}
        fn put_f64(&mut self, _n: f64) {}
    }

    let mut buf = TestBuf {
        data: vec![0; 1],
        position: 0,
    };

    buf.put_i16_le(1);
}

