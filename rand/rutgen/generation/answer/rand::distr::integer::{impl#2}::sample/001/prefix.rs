// Answer 0

#[test]
fn test_sample_with_default_rng() {
    struct MockRng {
        seed: u32,
        current: u32,
    }

    impl MockRng {
        fn new(seed: u32) -> Self {
            Self { seed, current: seed }
        }

        fn next_u32(&mut self) -> u32 {
            self.current = self.current.wrapping_add(1);
            self.current
        }
    }

    let mut rng = MockRng::new(0);
    let distribution = StandardUniform;
    let result = distribution.sample(&mut rng);
}

#[test]
fn test_sample_with_high_seed_rng() {
    struct MockRng {
        seed: u32,
        current: u32,
    }

    impl MockRng {
        fn new(seed: u32) -> Self {
            Self { seed, current: seed }
        }

        fn next_u32(&mut self) -> u32 {
            self.current = self.current.wrapping_add(1);
            self.current
        }
    }

    let mut rng = MockRng::new(4_294_967_295);
    let distribution = StandardUniform;
    let result = distribution.sample(&mut rng);
}

#[test]
fn test_sample_with_observed_range_rng() {
    struct MockRng {
        current: u32,
    }

    impl MockRng {
        fn new() -> Self {
            Self { current: 0 }
        }

        fn next_u32(&mut self) -> u32 {
            let result = self.current;
            self.current = self.current.wrapping_add(1);
            result
        }
    }

    let mut rng = MockRng::new();
    let distribution = StandardUniform;
    let result = distribution.sample(&mut rng); // Should cover values from 0 to 4,294,967,295
}

#[test]
fn test_sample_with_small_seed_rng() {
    struct MockRng {
        seed: u32,
        current: u32,
    }

    impl MockRng {
        fn new(seed: u32) -> Self {
            Self { seed, current: seed }
        }

        fn next_u32(&mut self) -> u32 {
            self.current = self.current.wrapping_add(1);
            self.current
        }
    }

    let mut rng = MockRng::new(1);
    let distribution = StandardUniform;
    let result = distribution.sample(&mut rng);
}

