// Answer 0

#[test]
fn test_random_ratio_case_1() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000_00000000_00000000_00000000 }; // All heads
    let mut coin_flipper = CoinFlipper::new(rng);
    let n = 1;
    let d = 4; // d > 2
    coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_2() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000_00000000_00000000_00000001 }; // Tails on first coin flip
    let mut coin_flipper = CoinFlipper::new(rng);
    let n = 1;
    let d = 5; // d > 2
    coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_3() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000_00000000_00000000_00000010 }; // Tails on first coin flip
    let mut coin_flipper = CoinFlipper::new(rng);
    let n = 3;
    let d = 8; // d > 2
    coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_4() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000_00000000_00000000_00000011 }; // Tails on first coin flip
    let mut coin_flipper = CoinFlipper::new(rng);
    let n = 2;
    let d = 7; // d > 2
    coin_flipper.random_ratio(n, d);
}

