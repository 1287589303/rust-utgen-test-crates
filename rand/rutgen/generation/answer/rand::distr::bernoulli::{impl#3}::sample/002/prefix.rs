// Answer 0

#[test]
fn test_sample_below_threshold() {
    struct TestRng {
        value: u64,
    }

    impl Rng for TestRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = TestRng { value: 10 };
    let bernoulli = Bernoulli { p_int: 20 };
    let result = bernoulli.sample(&mut rng);
}

#[test]
fn test_sample_equal_threshold() {
    struct TestRng {
        value: u64,
    }

    impl Rng for TestRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = TestRng { value: 20 };
    let bernoulli = Bernoulli { p_int: 20 };
    let result = bernoulli.sample(&mut rng);
}

#[test]
fn test_sample_above_threshold() {
    struct TestRng {
        value: u64,
    }

    impl Rng for TestRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = TestRng { value: 30 };
    let bernoulli = Bernoulli { p_int: 20 };
    let result = bernoulli.sample(&mut rng);
}

#[test]
fn test_sample_max_value() {
    struct TestRng {
        value: u64,
    }

    impl Rng for TestRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = TestRng { value: u64::MAX };
    let bernoulli = Bernoulli { p_int: u64::MAX - 1 };
    let result = bernoulli.sample(&mut rng);
}

