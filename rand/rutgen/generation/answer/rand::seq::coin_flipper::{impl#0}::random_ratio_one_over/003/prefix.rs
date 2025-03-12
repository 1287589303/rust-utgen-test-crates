// Answer 0

#[test]
#[should_panic]
fn test_random_ratio_one_over_zero() {
    let rng = TestRng::new(); 
    let mut coin_flipper = CoinFlipper::new(rng);
    coin_flipper.random_ratio_one_over(0); 
}

#[test]
fn test_random_ratio_one_over_min_non_zero_d() {
    let rng = TestRng::new_with_chunk(1); 
    let mut coin_flipper = CoinFlipper::new(rng);
    let result = coin_flipper.random_ratio_one_over(1); // leading_zeros(1) < 31
    assert_eq!(result, false); 
}

#[test]
fn test_random_ratio_one_over_max_non_zero_d() {
    let rng = TestRng::new_with_chunk(1); 
    let mut coin_flipper = CoinFlipper::new(rng);
    let result = coin_flipper.random_ratio_one_over(usize::MAX - 1); // leading_zeros(usize::MAX - 1) < 31
    assert_eq!(result, false); 
}

#[test]
fn test_random_ratio_one_over_mid_non_zero_d() {
    let rng = TestRng::new_with_chunk(1); 
    let mut coin_flipper = CoinFlipper::new(rng);
    let non_zero_d = 2; 
    let result = coin_flipper.random_ratio_one_over(non_zero_d); // leading_zeros(2) < 31
    assert_eq!(result, false); 
}

// Helper struct for the test
struct TestRng {
    chunk: u32,
}

impl TestRng {
    fn new() -> Self {
        Self { chunk: 0 }
    }
    
    fn new_with_chunk(chunk: u32) -> Self {
        Self { chunk }
    }
}

impl RngCore for TestRng {
    fn next_u32(&mut self) -> u32 {
        self.chunk
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.next_u32() as u8;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

