// Answer 0

#[test]
fn test_next_u64_index_zero() {
    struct MockBlockRngCore {
        results: [u64; 4],
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = [u64; 4];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let mut core = MockBlockRngCore { results: [1, 2, 3, 4] };
    let mut rng = BlockRng64 { results: [0; 4], index: 0, half_used: false, core };

    let result = rng.next_u64();
}

#[test]
fn test_next_u64_index_within_bounds() {
    struct MockBlockRngCore {
        results: [u64; 3],
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = [u64; 3];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let mut core = MockBlockRngCore { results: [5, 10, 15] };
    let mut rng = BlockRng64 { results: [0; 3], index: 1, half_used: false, core };

    let result = rng.next_u64();
}

#[test]
fn test_next_u64_index_at_max() {
    struct MockBlockRngCore {
        results: [u64; 2],
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = [u64; 2];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let mut core = MockBlockRngCore { results: [100, 200] };
    let mut rng = BlockRng64 { results: [0; 2], index: 1, half_used: false, core };

    let result = rng.next_u64();
}

