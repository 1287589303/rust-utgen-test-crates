// Answer 0

#[test]
fn test_block_rng_new_valid() {
    struct ValidBlockRngCore;

    impl RngCore for ValidBlockRngCore {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    struct ValidResults([u8; 0]);

    impl AsRef<[u8]> for ValidResults {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    impl AsMut<[u8]> for ValidResults {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut self.0
        }
    }

    impl Default for ValidResults {
        fn default() -> Self {
            ValidResults([0; 0])
        }
    }

    impl BlockRngCore for ValidBlockRngCore {
        type Item = u8;
        type Results = ValidResults;
        fn generate(&mut self, _results: &mut Self::Results) {}
    }

    let core = ValidBlockRngCore;
    let block_rng = BlockRng::new(core);
}

#[test]
fn test_block_rng_new_empty_results() {
    struct EmptyResultsCore;

    impl RngCore for EmptyResultsCore {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    struct EmptyResults;

    impl AsRef<[u8]> for EmptyResults {
        fn as_ref(&self) -> &[u8] {
            &[]
        }
    }

    impl AsMut<[u8]> for EmptyResults {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut []
        }
    }

    impl Default for EmptyResults {
        fn default() -> Self {
            EmptyResults
        }
    }

    impl BlockRngCore for EmptyResultsCore {
        type Item = u8;
        type Results = EmptyResults;
        fn generate(&mut self, _results: &mut Self::Results) {}
    }

    let core = EmptyResultsCore;
    let block_rng = BlockRng::new(core);
}

