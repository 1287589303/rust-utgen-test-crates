// Answer 0

#[test]
fn test_gen_ratio_zero_numerator() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> { Ok(()) }
    }
    impl Rng for TestRng {}

    let mut rng = TestRng;
    let _result = rng.gen_ratio(0, 1);
}

#[test]
fn test_gen_ratio_full_probability() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> { Ok(()) }
    }
    impl Rng for TestRng {}

    let mut rng = TestRng;
    let _result = rng.gen_ratio(UINT32_MAX, UINT32_MAX);
}

#[test]
fn test_gen_ratio_ratio_must_be_within_bounds() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> { Ok(()) }
    }
    impl Rng for TestRng {}

    let mut rng = TestRng;
    let _result = rng.gen_ratio(1, 2);
}

#[test]
#[should_panic(expected = "p=1/0 is outside range [0.0, 1.0]")]
fn test_gen_ratio_zero_denominator() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> { Ok(()) }
    }
    impl Rng for TestRng {}

    let mut rng = TestRng;
    let _result = rng.gen_ratio(1, 0);
}

