// Answer 0

#[test]
fn test_random_ratio_case_1() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0b00000000_00000000_00000000_00000000 // All tails
        }
    }

    let mut coin_flipper = CoinFlipper::new(DummyRng);
    let n = 2; // n == d/2
    let d = 4; // d being an even number greater than 0
    let result = coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_2() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0b00000000_00000000_00000000_00000000 // All tails
        }
    }

    let mut coin_flipper = CoinFlipper::new(DummyRng);
    let n = 3; // n == d/2
    let d = 6; // d being an even number greater than 0
    let result = coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_3() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0b00000000_00000000_00000000_00000000 // All tails
        }
    }

    let mut coin_flipper = CoinFlipper::new(DummyRng);
    let n = 4; // n == d/2
    let d = 8; // d being an even number greater than 0
    let result = coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_4() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0b00000000_00000000_00000000_00000000 // All tails
        }
    }

    let mut coin_flipper = CoinFlipper::new(DummyRng);
    let n = 5; // n == d/2
    let d = 10; // d being an even number greater than 0
    let result = coin_flipper.random_ratio(n, d);
}

