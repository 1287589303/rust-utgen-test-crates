// Answer 0

#[test]
fn test_fill_bytes_empty_slice() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_one_byte() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let mut rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest = [0u8; 1];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_two_bytes() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let mut rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest = [0u8; 2];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_255_bytes() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let mut rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest = [0u8; 255];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_256_bytes() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let mut rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest = [0u8; 256];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_1024_bytes() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let mut rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest = [0u8; 1024];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_4096_bytes() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let mut rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest = [0u8; 4096];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_65536_bytes() {
    struct TestRng;
    struct TestReseeder;

    impl BlockRngCore for TestRng {
        type Item = u32;
    }
    impl SeedableRng for TestRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }
    impl TryRngCore for TestReseeder {}

    let mut rng = ReseedingRng(BlockRng::from_rng(TestRng::from_seed([0u8; 32])));
    let mut dest = [0u8; 65536];
    rng.fill_bytes(&mut dest);
}

