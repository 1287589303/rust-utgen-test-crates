// Answer 0

#[test]
fn test_sample_valid_rng() {
    struct MockRng {
        next_value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.next_value
        }
    }

    let mut rng = MockRng { next_value: 1234567890 };
    let distribution = StandardUniform;

    let result = distribution.sample(&mut rng);
}

#[test]
fn test_sample_boundary_low() {
    struct MockRng {
        next_value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.next_value
        }
    }

    let mut rng = MockRng { next_value: 0 };
    let distribution = StandardUniform;

    let result = distribution.sample(&mut rng);
}

#[test]
fn test_sample_boundary_high() {
    struct MockRng {
        next_value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.next_value
        }
    }

    let mut rng = MockRng { next_value: 4294967295 }; // 2^32 - 1
    let distribution = StandardUniform;

    let result = distribution.sample(&mut rng);
}

