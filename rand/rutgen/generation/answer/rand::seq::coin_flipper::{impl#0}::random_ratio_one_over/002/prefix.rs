// Answer 0

#[test]
fn test_random_ratio_one_over_small_d() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 1 }; // controllable output
    let mut coin_flipper = CoinFlipper::new(rng);
    let d = 1; // minimal positive d
    coin_flipper.chunk = 0b11; // setting chunk to ensure flip_c_heads returns true
    coin_flipper.chunk_remaining = 2; // enough remaining bits

    let result = coin_flipper.random_ratio_one_over(d);
}

#[test]
fn test_random_ratio_one_over_large_d() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 2 }; // controllable output
    let mut coin_flipper = CoinFlipper::new(rng);
    let d = 4294967295; // maximum valid d
    coin_flipper.chunk = 0b11; // setting chunk to ensure flip_c_heads returns true
    coin_flipper.chunk_remaining = 2; // enough remaining bits

    let result = coin_flipper.random_ratio_one_over(d);
}

#[test]
#[should_panic]
fn test_random_ratio_one_over_zero_d() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 3 }; // controllable output
    let mut coin_flipper = CoinFlipper::new(rng);
    let d = 0; // d should not be zero

    let result = coin_flipper.random_ratio_one_over(d);
}

#[test]
fn test_random_ratio_one_over_mid_d() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 4 }; // controllable output
    let mut coin_flipper = CoinFlipper::new(rng);
    let d = 123456; // arbitrary valid d
    coin_flipper.chunk = 0b11; // setting chunk to ensure flip_c_heads returns true
    coin_flipper.chunk_remaining = 2; // enough remaining bits

    let result = coin_flipper.random_ratio_one_over(d);
}

