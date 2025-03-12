// Answer 0

#[test]
fn test_advance_a_rem_not_zero_a_rem_less_than_cnt() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let rem = self.remaining();
            let to_copy = std::cmp::min(rem, dst.len());
            dst[..to_copy].copy_from_slice(&self.data[self.position..self.position + to_copy]);
            self.position += to_copy;
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.position += 1;
            byte
        }

        fn get_i8(&mut self) -> i8 { unimplemented!() }
        fn get_u16(&mut self) -> u16 { unimplemented!() }
        fn get_u16_le(&mut self) -> u16 { unimplemented!() }
        fn get_u16_ne(&mut self) -> u16 { unimplemented!() }
        fn get_i16(&mut self) -> i16 { unimplemented!() }
        fn get_i16_le(&mut self) -> i16 { unimplemented!() }
        fn get_i16_ne(&mut self) -> i16 { unimplemented!() }
        fn get_u32(&mut self) -> u32 { unimplemented!() }
        fn get_u32_le(&mut self) -> u32 { unimplemented!() }
        fn get_u32_ne(&mut self) -> u32 { unimplemented!() }
        fn get_i32(&mut self) -> i32 { unimplemented!() }
        fn get_i32_le(&mut self) -> i32 { unimplemented!() }
        fn get_i32_ne(&mut self) -> i32 { unimplemented!() }
        fn get_u64(&mut self) -> u64 { unimplemented!() }
        fn get_u64_le(&mut self) -> u64 { unimplemented!() }
        fn get_u64_ne(&mut self) -> u64 { unimplemented!() }
        fn get_i64(&mut self) -> i64 { unimplemented!() }
        fn get_i64_le(&mut self) -> i64 { unimplemented!() }
        fn get_i64_ne(&mut self) -> i64 { unimplemented!() }
        fn get_u128(&mut self) -> u128 { unimplemented!() }
        fn get_u128_le(&mut self) -> u128 { unimplemented!() }
        fn get_u128_ne(&mut self) -> u128 { unimplemented!() }
        fn get_i128(&mut self) -> i128 { unimplemented!() }
        fn get_i128_le(&mut self) -> i128 { unimplemented!() }
        fn get_i128_ne(&mut self) -> i128 { unimplemented!() }
        fn get_uint(&mut self, nbytes: usize) -> u64 { unimplemented!() }
        fn get_uint_le(&mut self, nbytes: usize) -> u64 { unimplemented!() }
        fn get_uint_ne(&mut self, nbytes: usize) -> u64 { unimplemented!() }
        fn get_int(&mut self, nbytes: usize) -> i64 { unimplemented!() }
        fn get_int_le(&mut self, nbytes: usize) -> i64 { unimplemented!() }
        fn get_int_ne(&mut self, nbytes: usize) -> i64 { unimplemented!() }
        fn get_f32(&mut self) -> f32 { unimplemented!() }
        fn get_f32_le(&mut self) -> f32 { unimplemented!() }
        fn get_f32_ne(&mut self) -> f32 { unimplemented!() }
        fn get_f64(&mut self) -> f64 { unimplemented!() }
        fn get_f64_le(&mut self) -> f64 { unimplemented!() }
        fn get_f64_ne(&mut self) -> f64 { unimplemented!() }
        fn try_copy_to_slice(&mut self, _dst: &mut [u8]) -> Result<(), TryGetError> { unimplemented!() }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { unimplemented!() }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { unimplemented!() }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_uint(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_uint_le(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_uint_ne(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_int(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_int_le(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_int_ne(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
    }

    struct TestBuf2 {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf2 {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, _dst: &mut [u8]) { unimplemented!() }
        fn get_u8(&mut self) -> u8 { unimplemented!() }
        fn get_i8(&mut self) -> i8 { unimplemented!() }
        fn get_u16(&mut self) -> u16 { unimplemented!() }
        fn get_u16_le(&mut self) -> u16 { unimplemented!() }
        fn get_u16_ne(&mut self) -> u16 { unimplemented!() }
        fn get_i16(&mut self) -> i16 { unimplemented!() }
        fn get_i16_le(&mut self) -> i16 { unimplemented!() }
        fn get_i16_ne(&mut self) -> i16 { unimplemented!() }
        fn get_u32(&mut self) -> u32 { unimplemented!() }
        fn get_u32_le(&mut self) -> u32 { unimplemented!() }
        fn get_u32_ne(&mut self) -> u32 { unimplemented!() }
        fn get_i32(&mut self) -> i32 { unimplemented!() }
        fn get_i32_le(&mut self) -> i32 { unimplemented!() }
        fn get_i32_ne(&mut self) -> i32 { unimplemented!() }
        fn get_u64(&mut self) -> u64 { unimplemented!() }
        fn get_u64_le(&mut self) -> u64 { unimplemented!() }
        fn get_u64_ne(&mut self) -> u64 { unimplemented!() }
        fn get_i64(&mut self) -> i64 { unimplemented!() }
        fn get_i64_le(&mut self) -> i64 { unimplemented!() }
        fn get_i64_ne(&mut self) -> i64 { unimplemented!() }
        fn get_u128(&mut self) -> u128 { unimplemented!() }
        fn get_u128_le(&mut self) -> u128 { unimplemented!() }
        fn get_u128_ne(&mut self) -> u128 { unimplemented!() }
        fn get_i128(&mut self) -> i128 { unimplemented!() }
        fn get_i128_le(&mut self) -> i128 { unimplemented!() }
        fn get_i128_ne(&mut self) -> i128 { unimplemented!() }
        fn get_uint(&mut self, nbytes: usize) -> u64 { unimplemented!() }
        fn get_uint_le(&mut self, nbytes: usize) -> u64 { unimplemented!() }
        fn get_uint_ne(&mut self, nbytes: usize) -> u64 { unimplemented!() }
        fn get_int(&mut self, nbytes: usize) -> i64 { unimplemented!() }
        fn get_int_le(&mut self, nbytes: usize) -> i64 { unimplemented!() }
        fn get_int_ne(&mut self, nbytes: usize) -> i64 { unimplemented!() }
        fn get_f32(&mut self) -> f32 { unimplemented!() }
        fn get_f32_le(&mut self) -> f32 { unimplemented!() }
        fn get_f32_ne(&mut self) -> f32 { unimplemented!() }
        fn get_f64(&mut self) -> f64 { unimplemented!() }
        fn get_f64_le(&mut self) -> f64 { unimplemented!() }
        fn get_f64_ne(&mut self) -> f64 { unimplemented!() }
        fn try_copy_to_slice(&mut self, _dst: &mut [u8]) -> Result<(), TryGetError> { unimplemented!() }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { unimplemented!() }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { unimplemented!() }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_uint(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_uint_le(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_uint_ne(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_int(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_int_le(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_int_ne(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
    }

    let buf_a = TestBuf {
        data: vec![1, 2, 3, 4],
        position: 0,
    };

    let buf_b = TestBuf2 {
        data: vec![5, 6, 7, 8],
        position: 0,
    };

    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    let cnt = chain_buf.a.remaining() + 1; // Ensures a_rem < cnt
    chain_buf.advance(cnt);
}

