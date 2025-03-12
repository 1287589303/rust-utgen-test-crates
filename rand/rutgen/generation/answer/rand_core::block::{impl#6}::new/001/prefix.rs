// Answer 0

#[test]
fn test_new_block_rng_with_default_results() {
    struct MockRngCore;
    
    impl RngCore for MockRngCore {
        fn next_u32(&mut self) -> u32 {
            0
        }

        fn next_u64(&mut self) -> u64 {
            0
        }

        fn fill_bytes(&mut self, _: &mut [u8]) {}
    }

    struct MockBlockRngCore;

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.push(0);
        }
    }

    let core = MockBlockRngCore;
    let block_rng = BlockRng64::new(core);

    let expected_index = 0;
    assert_eq!(block_rng.index, expected_index);
    assert_eq!(block_rng.half_used, false);
}

#[test]
fn test_new_block_rng_with_empty_results() {
    struct EmptyResults;

    impl Default for EmptyResults {
        fn default() -> Self {
            EmptyResults
        }
    }

    impl AsRef<[u32]> for EmptyResults {
        fn as_ref(&self) -> &[u32] {
            &[]
        }
    }

    impl AsMut<[u32]> for EmptyResults {
        fn as_mut(&mut self) -> &mut [u32] {
            &mut []
        }
    }

    impl Default for EmptyResults {
        fn default() -> Self {
            EmptyResults
        }
    }

    struct MockBlockRngCore;

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = EmptyResults;

        fn generate(&mut self, _: &mut Self::Results) {}
    }

    let core = MockBlockRngCore;
    let block_rng = BlockRng64::new(core);

    let expected_index = 0;
    assert_eq!(block_rng.index, expected_index);
    assert_eq!(block_rng.half_used, false);
}

