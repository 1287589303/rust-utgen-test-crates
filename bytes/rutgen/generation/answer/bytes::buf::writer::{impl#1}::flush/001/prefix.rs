// Answer 0

#[test]
fn test_flush_with_empty_buffer() {
    struct EmptyBufMut;

    impl BufMut for EmptyBufMut {
        fn remaining_mut(&self) -> usize {
            0
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {}
        fn has_remaining_mut(&self) -> bool {
            false
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {}
        fn put_bytes(&mut self, val: u8, cnt: usize) {}
        fn put_u8(&mut self, n: u8) {}
        fn put_i8(&mut self, n: i8) {}
        fn put_u16(&mut self, n: u16) {}
        fn put_u16_le(&mut self, n: u16) {}
        fn put_u16_ne(&mut self, n: u16) {}
        fn put_i16(&mut self, n: i16) {}
        fn put_i16_le(&mut self, n: i16) {}
        fn put_i16_ne(&mut self, n: i16) {}
        fn put_u32(&mut self, n: u32) {}
        fn put_u32_le(&mut self, n: u32) {}
        fn put_u32_ne(&mut self, n: u32) {}
        fn put_i32(&mut self, n: i32) {}
        fn put_i32_le(&mut self, n: i32) {}
        fn put_i32_ne(&mut self, n: i32) {}
        fn put_u64(&mut self, n: u64) {}
        fn put_u64_le(&mut self, n: u64) {}
        fn put_u64_ne(&mut self, n: u64) {}
        fn put_i64(&mut self, n: i64) {}
        fn put_i64_le(&mut self, n: i64) {}
        fn put_i64_ne(&mut self, n: i64) {}
        fn put_u128(&mut self, n: u128) {}
        fn put_u128_le(&mut self, n: u128) {}
        fn put_u128_ne(&mut self, n: u128) {}
        fn put_i128(&mut self, n: i128) {}
        fn put_i128_le(&mut self, n: i128) {}
        fn put_i128_ne(&mut self, n: i128) {}
        fn put_uint(&mut self, n: u64, nbytes: usize) {}
        fn put_uint_le(&mut self, n: u64, nbytes: usize) {}
        fn put_uint_ne(&mut self, n: u64, nbytes: usize) {}
        fn put_int(&mut self, n: i64, nbytes: usize) {}
        fn put_int_le(&mut self, n: i64, nbytes: usize) {}
        fn put_int_ne(&mut self, n: i64, nbytes: usize) {}
        fn put_f32(&mut self, n: f32) {}
        fn put_f32_le(&mut self, n: f32) {}
        fn put_f32_ne(&mut self, n: f32) {}
        fn put_f64(&mut self, n: f64) {}
        fn put_f64_le(&mut self, n: f64) {}
        fn put_f64_ne(&mut self, n: f64) {}
    }

    let mut writer = Writer { buf: EmptyBufMut };
    writer.flush().unwrap();
}

#[test]
fn test_flush_with_full_buffer() {
    struct FullBufMut {
        data: [u8; 10],
        size: usize,
    }

    impl FullBufMut {
        fn new() -> Self {
            FullBufMut {
                data: [0; 10],
                size: 10,
            }
        }
    }

    impl BufMut for FullBufMut {
        fn remaining_mut(&self) -> usize {
            self.size
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.size = self.size.saturating_sub(cnt);
        }
        fn has_remaining_mut(&self) -> bool {
            self.size > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {}
        fn put_bytes(&mut self, val: u8, cnt: usize) {}
        fn put_u8(&mut self, n: u8) {}
        fn put_i8(&mut self, n: i8) {}
        fn put_u16(&mut self, n: u16) {}
        fn put_u16_le(&mut self, n: u16) {}
        fn put_u16_ne(&mut self, n: u16) {}
        fn put_i16(&mut self, n: i16) {}
        fn put_i16_le(&mut self, n: i16) {}
        fn put_i16_ne(&mut self, n: i16) {}
        fn put_u32(&mut self, n: u32) {}
        fn put_u32_le(&mut self, n: u32) {}
        fn put_u32_ne(&mut self, n: u32) {}
        fn put_i32(&mut self, n: i32) {}
        fn put_i32_le(&mut self, n: i32) {}
        fn put_i32_ne(&mut self, n: i32) {}
        fn put_u64(&mut self, n: u64) {}
        fn put_u64_le(&mut self, n: u64) {}
        fn put_u64_ne(&mut self, n: u64) {}
        fn put_i64(&mut self, n: i64) {}
        fn put_i64_le(&mut self, n: i64) {}
        fn put_i64_ne(&mut self, n: i64) {}
        fn put_u128(&mut self, n: u128) {}
        fn put_u128_le(&mut self, n: u128) {}
        fn put_u128_ne(&mut self, n: u128) {}
        fn put_i128(&mut self, n: i128) {}
        fn put_i128_le(&mut self, n: i128) {}
        fn put_i128_ne(&mut self, n: i128) {}
        fn put_uint(&mut self, n: u64, nbytes: usize) {}
        fn put_uint_le(&mut self, n: u64, nbytes: usize) {}
        fn put_uint_ne(&mut self, n: u64, nbytes: usize) {}
        fn put_int(&mut self, n: i64, nbytes: usize) {}
        fn put_int_le(&mut self, n: i64, nbytes: usize) {}
        fn put_int_ne(&mut self, n: i64, nbytes: usize) {}
        fn put_f32(&mut self, n: f32) {}
        fn put_f32_le(&mut self, n: f32) {}
        fn put_f32_ne(&mut self, n: f32) {}
        fn put_f64(&mut self, n: f64) {}
        fn put_f64_le(&mut self, n: f64) {}
        fn put_f64_ne(&mut self, n: f64) {}
    }

    let mut writer = Writer { buf: FullBufMut::new() };
    writer.flush().unwrap();
}

