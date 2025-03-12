// Answer 0

#[test]
fn test_from_rng_with_valid_rng() {
    struct TestRng {
        value: u32,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value += 1;
            self.value
        }

        fn next_u64(&mut self) -> u64 {
            self.value as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.next_u32() % 256) as u8;
            }
        }
    }

    impl Default for TestRng {
        fn default() -> Self {
            Self { value: 0 }
        }
    }

    let mut rng = TestRng::default();
    let _block_rng: BlockRng<TestRng> = BlockRng::from_rng(&mut rng);
}

#[test]
fn test_from_rng_with_another_rng() {
    struct AnotherTestRng {
        position: usize,
    }

    impl RngCore for AnotherTestRng {
        fn next_u32(&mut self) -> u32 {
            self.position as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.position as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for i in 0..dst.len() {
                dst[i] = (self.position % 256) as u8;
            }
        }
    }

    impl Default for AnotherTestRng {
        fn default() -> Self {
            Self { position: 0 }
        }
    }
    
    let mut rng = AnotherTestRng::default();
    let _block_rng: BlockRng<AnotherTestRng> = BlockRng::from_rng(&mut rng);
}

#[test]
fn test_from_rng_with_large_rng() {
    struct LargeTestRng {
        counter: u64,
    }

    impl RngCore for LargeTestRng {
        fn next_u32(&mut self) -> u32 {
            self.counter += 1;
            self.counter as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for i in 0..dst.len() {
                dst[i] = (self.counter % 256) as u8;
                self.counter += 1;
            }
        }
    }

    impl Default for LargeTestRng {
        fn default() -> Self {
            Self { counter: 0 }
        }
    }

    let mut rng = LargeTestRng::default();
    let _block_rng: BlockRng<LargeTestRng> = BlockRng::from_rng(&mut rng);
}

