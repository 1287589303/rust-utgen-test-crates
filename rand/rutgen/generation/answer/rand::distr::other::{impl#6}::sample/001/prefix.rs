// Answer 0

#[test]
fn test_sample_with_positive_value() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = TestRng { value: 1 }; // Positive value
    let distribution = StandardUniform;
    distribution.sample(&mut rng);
}

#[test]
fn test_sample_with_negative_value() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            0x80000000 // This will result in a negative value when cast to i32.
        }
    }

    let mut rng = TestRng { value: 0x80000000 };
    let distribution = StandardUniform;
    distribution.sample(&mut rng);
}

#[test]
fn test_sample_with_zero() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            0 // Zero value test
        }
    }

    let mut rng = TestRng { value: 0 };
    let distribution = StandardUniform;
    distribution.sample(&mut rng);
}

#[test]
fn test_sample_with_max_unsigned_value() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            u32::MAX // Maximum unsigned 32-bit integer test
        }
    }

    let mut rng = TestRng { value: u32::MAX };
    let distribution = StandardUniform;
    distribution.sample(&mut rng);
}

#[test]
fn test_sample_with_half_max_value() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            u32::MAX / 2 // Half of the maximum unsigned 32-bit integer
        }
    }

    let mut rng = TestRng { value: u32::MAX / 2 };
    let distribution = StandardUniform;
    distribution.sample(&mut rng);
}

