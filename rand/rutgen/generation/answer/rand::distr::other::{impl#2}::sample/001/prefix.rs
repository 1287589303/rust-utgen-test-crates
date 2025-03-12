// Answer 0

#[test]
fn test_sample_with_minimum_rng_value() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphanumeric = Alphanumeric;
    alphanumeric.sample(&mut rng);
}

#[test]
fn test_sample_with_mid_rng_value() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 31 };
    let alphanumeric = Alphanumeric;
    alphanumeric.sample(&mut rng);
}

#[test]
fn test_sample_with_maximum_rng_value() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 63 };
    let alphanumeric = Alphanumeric;
    alphanumeric.sample(&mut rng);
}

#[test]
fn test_sample_with_value_just_below_range() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 64 }; // This would be rejected, for coverage
    let alphanumeric = Alphanumeric;
    alphanumeric.sample(&mut rng);
}

