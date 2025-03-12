// Answer 0

#[test]
fn test_get_ref_with_vec() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize { self.data.capacity() - self.data.len() }
        unsafe fn advance_mut(&mut self, cnt: usize) { self.data.resize(self.data.len() + cnt, 0); }
        fn has_remaining_mut(&self) -> bool { self.remaining_mut() > 0 }
        fn chunk_mut(&mut self) -> &mut UninitSlice { unimplemented!() }
        fn put<T: super::Buf>(&mut self, _: T) { unimplemented!() }
        fn put_slice(&mut self, _: &[u8]) { unimplemented!() }
        fn put_bytes(&mut self, _: u8, _: usize) { unimplemented!() }
        fn put_u8(&mut self, _: u8) { unimplemented!() }
        fn put_i8(&mut self, _: i8) { unimplemented!() }
        fn put_u16(&mut self, _: u16) { unimplemented!() }
        fn put_u16_le(&mut self, _: u16) { unimplemented!() }
        fn put_u16_ne(&mut self, _: u16) { unimplemented!() }
        fn put_i16(&mut self, _: i16) { unimplemented!() }
        fn put_i16_le(&mut self, _: i16) { unimplemented!() }
        fn put_i16_ne(&mut self, _: i16) { unimplemented!() }
        fn put_u32(&mut self, _: u32) { unimplemented!() }
        fn put_u32_le(&mut self, _: u32) { unimplemented!() }
        fn put_u32_ne(&mut self, _: u32) { unimplemented!() }
        fn put_i32(&mut self, _: i32) { unimplemented!() }
        fn put_i32_le(&mut self, _: i32) { unimplemented!() }
        fn put_i32_ne(&mut self, _: i32) { unimplemented!() }
        fn put_u64(&mut self, _: u64) { unimplemented!() }
        fn put_u64_le(&mut self, _: u64) { unimplemented!() }
        fn put_u64_ne(&mut self, _: u64) { unimplemented!() }
        fn put_i64(&mut self, _: i64) { unimplemented!() }
        fn put_i64_le(&mut self, _: i64) { unimplemented!() }
        fn put_i64_ne(&mut self, _: i64) { unimplemented!() }
        fn put_u128(&mut self, _: u128) { unimplemented!() }
        fn put_u128_le(&mut self, _: u128) { unimplemented!() }
        fn put_u128_ne(&mut self, _: u128) { unimplemented!() }
        fn put_i128(&mut self, _: i128) { unimplemented!() }
        fn put_i128_le(&mut self, _: i128) { unimplemented!() }
        fn put_i128_ne(&mut self, _: i128) { unimplemented!() }
        fn put_uint(&mut self, _: u64, _: usize) { unimplemented!() }
        fn put_uint_le(&mut self, _: u64, _: usize) { unimplemented!() }
        fn put_uint_ne(&mut self, _: u64, _: usize) { unimplemented!() }
        fn put_int(&mut self, _: i64, _: usize) { unimplemented!() }
        fn put_int_le(&mut self, _: i64, _: usize) { unimplemented!() }
        fn put_int_ne(&mut self, _: i64, _: usize) { unimplemented!() }
        fn put_f32(&mut self, _: f32) { unimplemented!() }
        fn put_f32_le(&mut self, _: f32) { unimplemented!() }
        fn put_f32_ne(&mut self, _: f32) { unimplemented!() }
        fn put_f64(&mut self, _: f64) { unimplemented!() }
        fn put_f64_le(&mut self, _: f64) { unimplemented!() }
        fn put_f64_ne(&mut self, _: f64) { unimplemented!() }
    }

    let buf = TestBuf { data: Vec::with_capacity(1024) };
    let writer = Writer { buf };

    let ref_buf = writer.get_ref();
}

#[test]
fn test_get_ref_with_small_buffer() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize { self.data.capacity() - self.data.len() }
        unsafe fn advance_mut(&mut self, cnt: usize) { self.data.resize(self.data.len() + cnt, 0); }
        fn has_remaining_mut(&self) -> bool { self.remaining_mut() > 0 }
        fn chunk_mut(&mut self) -> &mut UninitSlice { unimplemented!() }
        fn put<T: super::Buf>(&mut self, _: T) { unimplemented!() }
        fn put_slice(&mut self, _: &[u8]) { unimplemented!() }
        fn put_bytes(&mut self, _: u8, _: usize) { unimplemented!() }
        fn put_u8(&mut self, _: u8) { unimplemented!() }
        fn put_i8(&mut self, _: i8) { unimplemented!() }
        fn put_u16(&mut self, _: u16) { unimplemented!() }
        fn put_u16_le(&mut self, _: u16) { unimplemented!() }
        fn put_u16_ne(&mut self, _: u16) { unimplemented!() }
        fn put_i16(&mut self, _: i16) { unimplemented!() }
        fn put_i16_le(&mut self, _: i16) { unimplemented!() }
        fn put_i16_ne(&mut self, _: i16) { unimplemented!() }
        fn put_u32(&mut self, _: u32) { unimplemented!() }
        fn put_u32_le(&mut self, _: u32) { unimplemented!() }
        fn put_u32_ne(&mut self, _: u32) { unimplemented!() }
        fn put_i32(&mut self, _: i32) { unimplemented!() }
        fn put_i32_le(&mut self, _: i32) { unimplemented!() }
        fn put_i32_ne(&mut self, _: i32) { unimplemented!() }
        fn put_u64(&mut self, _: u64) { unimplemented!() }
        fn put_u64_le(&mut self, _: u64) { unimplemented!() }
        fn put_u64_ne(&mut self, _: u64) { unimplemented!() }
        fn put_i64(&mut self, _: i64) { unimplemented!() }
        fn put_i64_le(&mut self, _: i64) { unimplemented!() }
        fn put_i64_ne(&mut self, _: i64) { unimplemented!() }
        fn put_u128(&mut self, _: u128) { unimplemented!() }
        fn put_u128_le(&mut self, _: u128) { unimplemented!() }
        fn put_u128_ne(&mut self, _: u128) { unimplemented!() }
        fn put_i128(&mut self, _: i128) { unimplemented!() }
        fn put_i128_le(&mut self, _: i128) { unimplemented!() }
        fn put_i128_ne(&mut self, _: i128) { unimplemented!() }
        fn put_uint(&mut self, _: u64, _: usize) { unimplemented!() }
        fn put_uint_le(&mut self, _: u64, _: usize) { unimplemented!() }
        fn put_uint_ne(&mut self, _: u64, _: usize) { unimplemented!() }
        fn put_int(&mut self, _: i64, _: usize) { unimplemented!() }
        fn put_int_le(&mut self, _: i64, _: usize) { unimplemented!() }
        fn put_int_ne(&mut self, _: i64, _: usize) { unimplemented!() }
        fn put_f32(&mut self, _: f32) { unimplemented!() }
        fn put_f32_le(&mut self, _: f32) { unimplemented!() }
        fn put_f32_ne(&mut self, _: f32) { unimplemented!() }
        fn put_f64(&mut self, _: f64) { unimplemented!() }
        fn put_f64_le(&mut self, _: f64) { unimplemented!() }
        fn put_f64_ne(&mut self, _: f64) { unimplemented!() }
    }

    let buf = TestBuf { data: Vec::with_capacity(16) };
    let writer = Writer { buf };

    let ref_buf = writer.get_ref();
}

