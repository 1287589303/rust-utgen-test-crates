// Answer 0

#[test]
fn test_index_zero() {
    struct MockBlockRngCore;
    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = [u32; 10]; // Assuming a results buffer of size 10

        fn generate(&mut self, _results: &mut Self::Results) {}
    }

    let core = MockBlockRngCore;
    let mut block_rng = BlockRng {
        results: [0; 10],
        index: 0,
        core,
    };

    let idx = block_rng.index();
}

#[test]
fn test_index_max() {
    struct MockBlockRngCore;
    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = [u32; 10]; // Assuming a results buffer of size 10

        fn generate(&mut self, _results: &mut Self::Results) {}
    }

    let core = MockBlockRngCore;
    let mut block_rng = BlockRng {
        results: [0; 10],
        index: 9, // set to max index
        core,
    };

    let idx = block_rng.index();
}

#[test]
#[should_panic] // this should panic because the index is equal to the size of the results buffer
fn test_index_equal_to_size() {
    struct MockBlockRngCore;
    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = [u32; 10]; // Assuming a results buffer of size 10

        fn generate(&mut self, _results: &mut Self::Results) {}
    }

    let core = MockBlockRngCore;
    let mut block_rng = BlockRng {
        results: [0; 10],
        index: 10, // this is equal to the size of results buffer
        core,
    };

    let idx = block_rng.index();
}

