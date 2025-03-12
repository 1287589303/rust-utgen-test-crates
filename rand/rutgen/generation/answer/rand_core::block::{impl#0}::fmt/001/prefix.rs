// Answer 0

#[test]
fn test_block_rng_fmt_non_empty() {
    struct TestRng {
        counter: u32,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.counter += 1;
            self.counter
        }
        
        fn next_u64(&mut self) -> u64 {
            (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
        }
        
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = self.next_u32() as u8;
            }
        }
    }

    struct TestSeed([u8; 16]);

    impl Default for TestSeed {
        fn default() -> Self {
            TestSeed([0; 16])
        }
    }

    impl AsRef<[u8]> for TestSeed {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    impl AsMut<[u8]> for TestSeed {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut self.0
        }
    }

    impl SeedableRng for TestRng {
        type Seed = TestSeed;

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng { counter: 0 }
        }
    }

    let mut rng = TestRng { counter: 0 };
    let mut results = [0u32; 10].as_mut();
    let block_rng = BlockRng {
        results,
        index: 0,
        core: rng,
    };
    
    let _ = format!("{:?}", block_rng);
}

#[test]
fn test_block_rng_fmt_with_index() {
    struct TestRng {
        counter: u32,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.counter += 1;
            self.counter
        }
        
        fn next_u64(&mut self) -> u64 {
            (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
        }
        
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = self.next_u32() as u8;
            }
        }
    }

    struct TestSeed([u8; 16]);

    impl Default for TestSeed {
        fn default() -> Self {
            TestSeed([0; 16])
        }
    }

    impl AsRef<[u8]> for TestSeed {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    impl AsMut<[u8]> for TestSeed {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut self.0
        }
    }

    impl SeedableRng for TestRng {
        type Seed = TestSeed;

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng { counter: 0 }
        }
    }

    let mut rng = TestRng { counter: 0 };
    let mut results = [0u32; 10].as_mut();
    let mut block_rng = BlockRng {
        results,
        index: 5,
        core: rng,
    };

    let _ = format!("{:?}", block_rng);
}

