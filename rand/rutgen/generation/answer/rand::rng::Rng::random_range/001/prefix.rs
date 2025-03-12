// Answer 0

#[test]
#[should_panic(expected = "cannot sample empty range")]
fn test_random_range_empty_exclusive_range_u32() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> rand_core::Result<()> { Ok(()) }
    }

    let mut rng = TestRng;

    // Exclusive range where low >= high
    let _: u32 = rng.random_range(10..10);
}

#[test]
#[should_panic(expected = "cannot sample empty range")]
fn test_random_range_empty_inclusive_range_u32() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> rand_core::Result<()> { Ok(()) }
    }

    let mut rng = TestRng;

    // Inclusive range where high < low
    let _: u32 = rng.random_range(10..=5);
}

#[test]
#[should_panic(expected = "cannot sample empty range")]
fn test_random_range_empty_exclusive_range_f64() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> rand_core::Result<()> { Ok(()) }
    }

    let mut rng = TestRng;

    // Exclusive range where low >= high
    let _: f64 = rng.random_range(1.0..1.0);
}

#[test]
#[should_panic(expected = "cannot sample empty range")]
fn test_random_range_empty_inclusive_range_f64() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> rand_core::Result<()> { Ok(()) }
    }

    let mut rng = TestRng;

    // Inclusive range where high < low
    let _: f64 = rng.random_range(1.0..=0.0);
}

