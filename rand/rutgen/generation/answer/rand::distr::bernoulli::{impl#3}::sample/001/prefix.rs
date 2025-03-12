// Answer 0

#[test]
fn test_sample_always_true() {
    struct MockRng {
        value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let rng = &mut MockRng { value: 0 };
    let bernoulli = Bernoulli { p_int: u64::MAX };
    let result = bernoulli.sample(rng);
}

#[test]
fn test_sample_always_true_with_different_rng() {
    struct AnotherMockRng {
        counter: u64,
    }

    impl Rng for AnotherMockRng {
        fn random(&mut self) -> u64 {
            self.counter += 1;
            self.counter
        }
    }

    let rng = &mut AnotherMockRng { counter: 0 };
    let bernoulli = Bernoulli { p_int: u64::MAX };
    let result = bernoulli.sample(rng);
}

