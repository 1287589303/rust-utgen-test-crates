// Answer 0

#[test]
fn test_next_with_non_empty_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn get_u8(&mut self) -> u8 { unimplemented!() }
        fn get_i8(&mut self) -> i8 { unimplemented!() }
        fn get_u16(&mut self) -> u16 { unimplemented!() }
        fn get_u32(&mut self) -> u32 { unimplemented!() }
        fn get_u64(&mut self) -> u64 { unimplemented!() }
        fn get_u128(&mut self) -> u128 { unimplemented!() }
        fn get_i16(&mut self) -> i16 { unimplemented!() }
        fn get_i32(&mut self) -> i32 { unimplemented!() }
        fn get_i64(&mut self) -> i64 { unimplemented!() }
        fn get_i128(&mut self) -> i128 { unimplemented!() }
        fn get_f32(&mut self) -> f32 { unimplemented!() }
        fn get_f64(&mut self) -> f64 { unimplemented!() }
        fn copy_to_slice(&mut self, dst: &mut [u8]) { unimplemented!() }
        fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), TryGetError> { unimplemented!() }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { unimplemented!() }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { unimplemented!() }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes { unimplemented!() }
    }

    let mut buf = TestBuf { data: vec![1, 2, 3], pos: 0 };
    let mut iter = IntoIter { inner: buf };
    
    let result = iter.next();
}

#[test]
fn test_next_with_partially_consumed_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn get_u8(&mut self) -> u8 { unimplemented!() }
        fn get_i8(&mut self) -> i8 { unimplemented!() }
        fn get_u16(&mut self) -> u16 { unimplemented!() }
        fn get_u32(&mut self) -> u32 { unimplemented!() }
        fn get_u64(&mut self) -> u64 { unimplemented!() }
        fn get_u128(&mut self) -> u128 { unimplemented!() }
        fn get_i16(&mut self) -> i16 { unimplemented!() }
        fn get_i32(&mut self) -> i32 { unimplemented!() }
        fn get_i64(&mut self) -> i64 { unimplemented!() }
        fn get_i128(&mut self) -> i128 { unimplemented!() }
        fn get_f32(&mut self) -> f32 { unimplemented!() }
        fn get_f64(&mut self) -> f64 { unimplemented!() }
        fn copy_to_slice(&mut self, dst: &mut [u8]) { unimplemented!() }
        fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), TryGetError> { unimplemented!() }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { unimplemented!() }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { unimplemented!() }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { unimplemented!() }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { unimplemented!() }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { unimplemented!() }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { unimplemented!() }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { unimplemented!() }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { unimplemented!() }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { unimplemented!() }
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes { unimplemented!() }
    }

    let mut buf = TestBuf { data: vec![4, 5, 6], pos: 1 };
    let mut iter = IntoIter { inner: buf };
    
    let result = iter.next();
}

