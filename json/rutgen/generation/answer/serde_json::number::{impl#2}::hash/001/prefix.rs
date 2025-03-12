// Answer 0

#[test]
fn test_hash_float_zero_positive() {
    struct TestHasher;

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 { 0 }
        fn write(&mut self, _: &[u8]) {}
        fn write_u8(&mut self, _: u8) {}
        fn write_u16(&mut self, _: u16) {}
        fn write_u32(&mut self, _: u32) {}
        fn write_u64(&mut self, _: u64) {}
        fn write_usize(&mut self, _: usize) {}
        fn write_i8(&mut self, _: i8) {}
        fn write_i16(&mut self, _: i16) {}
        fn write_i32(&mut self, _: i32) {}
        fn write_i64(&mut self, _: i64) {}
        fn write_isize(&mut self, _: isize) {}
        fn write_f32(&mut self, _: f32) {}
        fn write_f64(&mut self, _: f64) {}
    }

    let value = N::Float(0.0);
    let mut hasher = TestHasher;
    value.hash(&mut hasher);
}

#[test]
fn test_hash_float_zero_negative() {
    struct TestHasher;

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 { 0 }
        fn write(&mut self, _: &[u8]) {}
        fn write_u8(&mut self, _: u8) {}
        fn write_u16(&mut self, _: u16) {}
        fn write_u32(&mut self, _: u32) {}
        fn write_u64(&mut self, _: u64) {}
        fn write_usize(&mut self, _: usize) {}
        fn write_i8(&mut self, _: i8) {}
        fn write_i16(&mut self, _: i16) {}
        fn write_i32(&mut self, _: i32) {}
        fn write_i64(&mut self, _: i64) {}
        fn write_isize(&mut self, _: isize) {}
        fn write_f32(&mut self, _: f32) {}
        fn write_f64(&mut self, _: f64) {}
    }

    let value = N::Float(-0.0);
    let mut hasher = TestHasher;
    value.hash(&mut hasher);
}

