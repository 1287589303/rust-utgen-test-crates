// Answer 0

#[test]
fn test_flip_c_heads_case_c_equals_32() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000000000000000000000000000 };
    let mut flipper = CoinFlipper::new(rng);
    flipper.chunk = 0b00000000000000000000000000000000; // 32 leading zeros
    flipper.chunk_remaining = 32;

    flipper.flip_c_heads(32);
}

#[test]
fn test_flip_c_heads_case_c_is_31() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000000000000000000000000000 };
    let mut flipper = CoinFlipper::new(rng);
    flipper.chunk = 0b00000000000000000000000000000000; // 31 leading zeros
    flipper.chunk_remaining = 32;

    let result = flipper.flip_c_heads(31);
}

#[test]
fn test_flip_c_heads_case_c_is_30() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000000000000000000000000000 };
    let mut flipper = CoinFlipper::new(rng);
    flipper.chunk = 0b00000000000000000000000000000000; // 30 leading zeros
    flipper.chunk_remaining = 32;

    let result = flipper.flip_c_heads(30);
}

#[test]
fn test_flip_c_heads_case_c_is_1() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b00000000000000000000000000000000 };
    let mut flipper = CoinFlipper::new(rng);
    flipper.chunk = 0b00000000000000000000000000000000; // 1 leading zero
    flipper.chunk_remaining = 32;

    let result = flipper.flip_c_heads(1);
}

