// Answer 0

#[test]
fn test_fill_bytes_non_empty_dest_length() {
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

    let mut test_core = TestCore { data: vec![1, 2, 3, 4] };
    let mut results = vec![0; 4];
    let mut block_rng = BlockRng::new(test_core);

    let mut dest: [u8; 8] = [0; 8];
    block_rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_with_full_dest_usage() {
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

    let mut test_core = TestCore { data: vec![1] }; // Only one u32 to fill the whole array.
    let mut results = vec![0; 1];
    let mut block_rng = BlockRng::new(test_core);

    let mut dest: [u8; 4] = [0; 4];
    block_rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_when_read_len_equals_dest_length() {
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

    let mut test_core = TestCore { data: vec![1, 2] };
    let mut results = vec![0; 2];
    let mut block_rng = BlockRng::new(test_core);
    
    let mut dest: [u8; 8] = [0; 8];  // Enough room for two u32 values.
    block_rng.fill_bytes(&mut dest);
}

