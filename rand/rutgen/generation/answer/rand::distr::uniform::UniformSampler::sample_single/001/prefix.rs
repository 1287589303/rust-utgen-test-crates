// Answer 0

#[test]
fn test_sample_single_empty_range_low_greater_high() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { Ok(()) }
    }

    let mut rng = TestRng;
    let low = 5;
    let high = 3;
    let result = <UniformInt as UniformSampler>::sample_single(low, high, &mut rng);
}

#[test]
fn test_sample_single_non_finite_low() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { Ok(()) }
    }

    let mut rng = TestRng;
    let low = f32::NAN;
    let high = 3.0;
    let result = <UniformFloat as UniformSampler>::sample_single(low, high, &mut rng);
}

#[test]
fn test_sample_single_non_finite_high() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { Ok(()) }
    }

    let mut rng = TestRng;
    let low = 1.0;
    let high = f32::NAN;
    let result = <UniformFloat as UniformSampler>::sample_single(low, high, &mut rng);
}

#[test]
fn test_sample_single_equal_low_high() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { Ok(()) }
    }

    let mut rng = TestRng;
    let low = 4;
    let high = 4;
    let result = <UniformInt as UniformSampler>::sample_single(low, high, &mut rng);
}

#[test]
fn test_sample_single_inclusive_equal_low_high() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { Ok(()) }
    }

    let mut rng = TestRng;
    let low = 2.0;
    let high = 2.0;
    let result = <UniformFloat as UniformSampler>::sample_single_inclusive(low, high, &mut rng);
}

