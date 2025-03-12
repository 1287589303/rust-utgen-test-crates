// Answer 0

#[test]
fn test_random_ratio_equal_n_and_d() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0b11111111_11111111_11111111_11111111 // Simulate all heads
        }
    }
    
    let mut rng = MockRng;
    let mut flipper = CoinFlipper::new(rng);
    let n = 10;
    let d = 10;
    flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_equal_n_and_d_large() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0b11111111_11111111_11111111_11111111 // Simulate all heads
        }
    }
    
    let mut rng = MockRng;
    let mut flipper = CoinFlipper::new(rng);
    let n = usize::MAX;
    let d = usize::MAX;
    flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_boundary_case() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0b11111111_11111111_11111111_11111111 // Simulate all heads
        }
    }
    
    let mut rng = MockRng;
    let mut flipper = CoinFlipper::new(rng);
    let n = 1;
    let d = 1;
    flipper.random_ratio(n, d);
}

