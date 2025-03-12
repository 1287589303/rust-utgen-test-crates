// Answer 0

#[test]
fn test_seed_from_u64_zero() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
    }

    struct DummyBlockRngCore;
    impl BlockRngCore for DummyBlockRngCore {
        type Item = u8;
        type Results = [u8; 32];
        fn generate(&mut self, results: &mut Self::Results) {
            for byte in results.iter_mut() {
                *byte = 0;
            }
        }
    }

    let _rng = BlockRng64::<DummyBlockRngCore>::seed_from_u64(0);
}

#[test]
fn test_seed_from_u64_one() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 1 }
        fn next_u64(&mut self) -> u64 { 1 }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
    }

    struct DummyBlockRngCore;
    impl BlockRngCore for DummyBlockRngCore {
        type Item = u8;
        type Results = [u8; 32];
        fn generate(&mut self, results: &mut Self::Results) {
            for byte in results.iter_mut() {
                *byte = 1;
            }
        }
    }

    let _rng = BlockRng64::<DummyBlockRngCore>::seed_from_u64(1);
}

#[test]
fn test_seed_from_u64_max() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { u32::MAX }
        fn next_u64(&mut self) -> u64 { u64::MAX }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
    }

    struct DummyBlockRngCore;
    impl BlockRngCore for DummyBlockRngCore {
        type Item = u8;
        type Results = [u8; 32];
        fn generate(&mut self, results: &mut Self::Results) {
            for byte in results.iter_mut() {
                *byte = u8::MAX;
            }
        }
    }

    let _rng = BlockRng64::<DummyBlockRngCore>::seed_from_u64(u64::MAX);
}

#[test]
fn test_seed_from_u64_typical() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 100 }
        fn next_u64(&mut self) -> u64 { 100 }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
    }

    struct DummyBlockRngCore;
    impl BlockRngCore for DummyBlockRngCore {
        type Item = u8;
        type Results = [u8; 32];
        fn generate(&mut self, results: &mut Self::Results) {
            for byte in results.iter_mut() {
                *byte = 100;
            }
        }
    }

    let _rng = BlockRng64::<DummyBlockRngCore>::seed_from_u64(100);
}

#[test]
fn test_seed_from_u64_large_typical() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 1234567890 }
        fn next_u64(&mut self) -> u64 { 1234567890 }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
    }

    struct DummyBlockRngCore;
    impl BlockRngCore for DummyBlockRngCore {
        type Item = u8;
        type Results = [u8; 32];
        fn generate(&mut self, results: &mut Self::Results) {
            for byte in results.iter_mut() {
                *byte = 90;
            }
        }
    }

    let _rng = BlockRng64::<DummyBlockRngCore>::seed_from_u64(1234567890);
}

