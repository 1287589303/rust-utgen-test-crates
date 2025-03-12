// Answer 0

#[test]
fn test_flip_c_heads_boundaries() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0b00000000000000000000000000000000 // Return an all-zeros chunk
        }
    }

    let mut rng = TestRng;
    let mut coin_flipper = CoinFlipper::new(rng);
    coin_flipper.chunk_remaining = 32; // Set chunk_remaining to 32
    let result = coin_flipper.flip_c_heads(32); // Set c = 32
    // result is expected to be true
}

#[test]
fn test_flip_c_heads_zeros_equal_c() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0b00000000000000000000000000000000 // Return an all-zeros chunk
        }
    }

    let mut rng = TestRng;
    let mut coin_flipper = CoinFlipper::new(rng);
    coin_flipper.chunk_remaining = 32; // Set chunk_remaining to 32
    coin_flipper.chunk = 0b00000000000000000000000000000000; // Set chunk to all zeros
    let result = coin_flipper.flip_c_heads(32); // Set c = 32
    // result is expected to be true
}

