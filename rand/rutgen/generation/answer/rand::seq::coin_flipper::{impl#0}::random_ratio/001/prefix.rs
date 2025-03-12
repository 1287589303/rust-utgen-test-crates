// Answer 0

#[test]
fn test_random_ratio_case_n_less_d() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b11111111 };
    let mut flipper = CoinFlipper::new(rng);
    let result = flipper.random_ratio(10, 20);
}

#[test]
fn test_random_ratio_case_n_equals_d() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b11111111 };
    let mut flipper = CoinFlipper::new(rng);
    let result = flipper.random_ratio(15, 15);
}

