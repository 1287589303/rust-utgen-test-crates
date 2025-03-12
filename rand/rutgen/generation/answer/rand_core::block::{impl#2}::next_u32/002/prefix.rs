// Answer 0

#[test]
fn test_next_u32_with_valid_index() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }

    impl Default for MockBlockRngCore {
        fn default() -> Self {
            Self { results: vec![1, 2, 3, 4, 5] }
        }
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = MockBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 0;

    let result = block_rng.next_u32();
}

#[test]
fn test_next_u32_with_middle_index() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }

    impl Default for MockBlockRngCore {
        fn default() -> Self {
            Self { results: vec![10, 20, 30, 40, 50] }
        }
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = MockBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 2;

    let result = block_rng.next_u32();
}

#[test]
fn test_next_u32_with_last_index() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }

    impl Default for MockBlockRngCore {
        fn default() -> Self {
            Self { results: vec![100, 200, 300] }
        }
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = MockBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 2;

    let result = block_rng.next_u32();
}

#[test]
fn test_next_u32_with_zero_index() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }

    impl Default for MockBlockRngCore {
        fn default() -> Self {
            Self { results: vec![7, 14, 21] }
        }
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = MockBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 0;

    let result = block_rng.next_u32();
}

