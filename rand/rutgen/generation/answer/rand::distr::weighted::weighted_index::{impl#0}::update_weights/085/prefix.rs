// Answer 0

#[test]
fn test_update_weights_valid_case() {
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = TestWeight;
        fn new(_: TestWeight, _: TestWeight) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0.into(), 1.0.into(), 3.0.into()],
        total_weight: 6.0.into(),
        weight_distribution: TestSampler::new(TestWeight(0.0), TestWeight(6.0)).unwrap(),
    };

    let new_weights = [(0, &TestWeight(1.0)), (1, &TestWeight(2.0)), (2, &TestWeight(3.0))];
    weighted_index.update_weights(&new_weights).unwrap();
}

#[test]
fn test_update_weights_empty_new_weights() {
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = TestWeight;
        fn new(_: TestWeight, _: TestWeight) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0.into(), 1.0.into(), 3.0.into()],
        total_weight: 6.0.into(),
        weight_distribution: TestSampler::new(TestWeight(0.0), TestWeight(6.0)).unwrap(),
    };

    let new_weights: [(usize, &TestWeight)] = [];
    weighted_index.update_weights(&new_weights).unwrap();
}

#[test]
fn test_update_weights_invalid_weight_negative() {
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = TestWeight;
        fn new(_: TestWeight, _: TestWeight) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0.into(), 1.0.into(), 3.0.into()],
        total_weight: 6.0.into(),
        weight_distribution: TestSampler::new(TestWeight(0.0), TestWeight(6.0)).unwrap(),
    };

    let new_weights = [(0, &TestWeight(-1.0))];
    assert!(weighted_index.update_weights(&new_weights).is_err());
}

#[test]
fn test_update_weights_invalid_input_ordering() {
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = TestWeight;
        fn new(_: TestWeight, _: TestWeight) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0.into(), 1.0.into(), 3.0.into()],
        total_weight: 6.0.into(),
        weight_distribution: TestSampler::new(TestWeight(0.0), TestWeight(6.0)).unwrap(),
    };

    let new_weights = [(1, &TestWeight(2.0)), (0, &TestWeight(1.0))];
    assert!(weighted_index.update_weights(&new_weights).is_err());
}

#[test]
fn test_update_weights_insufficient_non_zero_weight() {
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = TestWeight;
        fn new(_: TestWeight, _: TestWeight) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0.into(), 0.0.into(), 0.0.into()],
        total_weight: 0.0.into(),
        weight_distribution: TestSampler::new(TestWeight(0.0), TestWeight(6.0)).unwrap(),
    };

    let new_weights = [(0, &TestWeight(0.0))];
    assert!(weighted_index.update_weights(&new_weights).is_err());
}

