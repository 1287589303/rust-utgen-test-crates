// Answer 0

#[test]
fn test_fill_bytes_index_at_results_length_non_empty_dest() {
    struct TestCore {
        data: Vec<u32>,
    }
    
    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = Vec<u32>;
        
        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.data);
        }
    }

    let core = TestCore { data: vec![1, 2, 3, 4] };
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 4; // set index to results length

    let mut dest = vec![0u8; 8]; // dest size < 12 (number of bytes for 3 u32)
    block_rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_index_at_results_length_empty_dest() {
    struct TestCore {
        data: Vec<u32>,
    }
    
    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = Vec<u32>;
        
        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.data);
        }
    }

    let core = TestCore { data: vec![1, 2, 3, 4] };
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 4; // set index to results length

    let mut dest = vec![]; // empty dest
    block_rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_index_exceeds_results_length() {
    struct TestCore {
        data: Vec<u32>,
    }
    
    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = Vec<u32>;
        
        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.data);
        }
    }

    let core = TestCore { data: vec![1, 2, 3, 4] };
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 4; // set index to results length

    let mut dest = vec![0u8; 12]; // dest size for 3 u32
    block_rng.fill_bytes(&mut dest);
}

