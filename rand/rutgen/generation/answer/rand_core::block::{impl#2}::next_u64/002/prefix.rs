// Answer 0

#[test]
fn test_next_u64_index_equals_len_minus_one() {
    struct TestCore {
        data: Vec<u32>,
    }

    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend_from_slice(&self.data);
        }
    }

    let mut core = TestCore { data: vec![10, 20] };
    let mut rng = BlockRng::new(core);
    rng.index = 1; // setting index to len - 1 where len = 2
    rng.results = vec![10, 20];

    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_index_greater_than_len() {
    struct TestCore {
        data: Vec<u32>,
    }

    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend_from_slice(&self.data);
        }
    }

    let mut core = TestCore { data: vec![10, 20] };
    let mut rng = BlockRng::new(core);
    rng.index = 2; // setting index to len where len = 2
    rng.results = vec![10, 20];

    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_results_length_zero() {
    struct TestCore {
        data: Vec<u32>,
    }

    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.clear();
        }
    }

    let mut core = TestCore { data: vec![] };
    let mut rng = BlockRng::new(core);
    rng.index = 0; // starting from index
    rng.results = vec![];

    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_results_length_greater_than_two() {
    struct TestCore {
        data: Vec<u32>,
    }

    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend_from_slice(&self.data);
        }
    }

    let mut core = TestCore { data: vec![10, 20, 30, 40] };
    let mut rng = BlockRng::new(core);
    rng.index = 2; // setting index to len - 1 where len = 4
    rng.results = vec![10, 20, 30, 40];

    let _result = rng.next_u64();
}

