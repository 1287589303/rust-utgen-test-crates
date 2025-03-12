// Answer 0

#[test]
fn test_thread_rng_debug_fmt() {
    struct MockRng;
    impl CryptoRng for MockRng {}
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 { 42 }
        fn next_u64(&mut self) -> u64 { 42 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> { Ok(()) }
    }
    
    struct MockReseedingCore;
    impl BlockRngCore for MockReseedingCore {}
    
    struct MockSeedableRng;
    impl SeedableRng for MockSeedableRng {
        type Seed = [u8; 32];
        fn from_seed(_seed: Self::Seed) -> Self { Self }
    }
    
    let rng = Rc::new(UnsafeCell::new(ReseedingRng(MockBlockRng(MockRng))));
    let thread_rng = ThreadRng { rng };
    let mut fmt = fmt::Formatter::new();

    let _ = thread_rng.fmt(&mut fmt);
}

#[test]
fn test_thread_rng_debug_fmt_thread_safety() {
    struct TestRng;
    impl CryptoRng for TestRng {}
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 37 }
        fn next_u64(&mut self) -> u64 { 37 }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> { Ok(()) }
    }
    
    struct TestReseedingCore;
    impl BlockRngCore for TestReseedingCore {}

    struct TestSeedableRng;
    impl SeedableRng for TestSeedableRng {
        type Seed = [u8; 32];
        fn from_seed(_seed: Self::Seed) -> Self { Self }
    }
    
    let rng = Rc::new(UnsafeCell::new(ReseedingRng(MockBlockRng(TestRng))));
    let thread_rng = ThreadRng { rng };
    let mut fmt = fmt::Formatter::new();

    let _ = thread_rng.fmt(&mut fmt);
}

