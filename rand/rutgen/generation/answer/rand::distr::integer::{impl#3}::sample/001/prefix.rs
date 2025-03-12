// Answer 0

#[test]
fn test_sample_valid_rng() {
    struct MockRng {
        value: u64,
    }

    impl Rng for MockRng {
        fn next_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = MockRng { value: 42 };
    let distr = StandardUniform;
    let result = distr.sample(&mut rng);
}

#[test]
fn test_sample_rng_altering_state() {
    struct StatefulRng {
        next_value: u64,
    }

    impl Rng for StatefulRng {
        fn next_u64(&mut self) -> u64 {
            let current = self.next_value;
            self.next_value += 1;
            current
        }
    }

    let mut rng = StatefulRng { next_value: 100 };
    let distr = StandardUniform;
    let result1 = distr.sample(&mut rng);
    let result2 = distr.sample(&mut rng);
}

#[test]
fn test_sample_rng_zero() {
    struct ZeroRng;

    impl Rng for ZeroRng {
        fn next_u64(&mut self) -> u64 {
            0
        }
    }

    let mut rng = ZeroRng;
    let distr = StandardUniform;
    let result = distr.sample(&mut rng);
}

#[test]
fn test_sample_rng_large_number() {
    struct LargeValueRng {
        value: u64,
    }

    impl Rng for LargeValueRng {
        fn next_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = LargeValueRng { value: u64::MAX };
    let distr = StandardUniform;
    let result = distr.sample(&mut rng);
}

