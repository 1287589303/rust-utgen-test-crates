// Answer 0

#[test]
fn test_random_ratio_case_numerator_zero() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::error::Error> { Ok(()) }
    }

    impl Rng for TestRng {}

    let mut rng = TestRng;
    let result = rng.random_ratio(0, 1);
}

#[test]
fn test_random_ratio_case_numerator_equals_denominator() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::error::Error> { Ok(()) }
    }

    impl Rng for TestRng {}

    let mut rng = TestRng;
    let result = rng.random_ratio(1, 1);
}

#[test]
fn test_random_ratio_case_valid_probability() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::error::Error> { Ok(()) }
    }

    impl Rng for TestRng {}

    let mut rng = TestRng;
    let result = rng.random_ratio(2, 3);
}

#[test]
#[should_panic]
fn test_random_ratio_case_denominator_zero() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::error::Error> { Ok(()) }
    }

    impl Rng for TestRng {}

    let mut rng = TestRng;
    let result = rng.random_ratio(2, 0);
}

#[test]
#[should_panic]
fn test_random_ratio_case_numerator_exceeds_denominator() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::error::Error> { Ok(()) }
    }

    impl Rng for TestRng {}

    let mut rng = TestRng;
    let result = rng.random_ratio(3, 2);
}

