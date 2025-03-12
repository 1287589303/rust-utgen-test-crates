// Answer 0

#[test]
fn test_try_next_u64_zero() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = TestRng;
    let _result = rng.try_next_u64();
}

#[test]
fn test_try_next_u64_max() {
    struct MaxRng;

    impl RngCore for MaxRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { u64::MAX }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = MaxRng;
    let _result = rng.try_next_u64();
}

#[test]
fn test_try_next_u64_mid() {
    struct MidRng;

    impl RngCore for MidRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 1234567890123456789 }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = MidRng;
    let _result = rng.try_next_u64();
}

