// Answer 0

#[test]
fn test_update_weights_valid() {
    struct TestWeight {
        value: f64,
    }

    impl SampleUniform for TestWeight {
        type Sampler = ();
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight { value: 1.0 }, TestWeight { value: 2.0 }],
        total_weight: TestWeight { value: 3.0 },
        weight_distribution: (),
    };

    let new_weights = vec![(0, &TestWeight { value: 0.0 })];

    let result = weighted_index.update_weights(&new_weights);
    assert!(result.is_ok());
}

#[test]
fn test_update_weights_with_empty_new_weights() {
    struct TestWeight {
        value: f64,
    }

    impl SampleUniform for TestWeight {
        type Sampler = ();
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight { value: 1.0 }, TestWeight { value: 2.0 }],
        total_weight: TestWeight { value: 3.0 },
        weight_distribution: (),
    };

    let new_weights: Vec<(usize, &TestWeight)> = vec![];

    let result = weighted_index.update_weights(&new_weights);
    assert!(result.is_ok());
}

#[test]
fn test_update_weights_with_invalid_weight() {
    struct TestWeight {
        value: f64,
    }

    impl SampleUniform for TestWeight {
        type Sampler = ();
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight { value: 1.0 }, TestWeight { value: 2.0 }],
        total_weight: TestWeight { value: 3.0 },
        weight_distribution: (),
    };

    let new_weights = vec![(1, &TestWeight { value: -0.5 })];

    let result = weighted_index.update_weights(&new_weights);
    assert!(matches!(result, Err(Error::InvalidWeight)));
}

#[test]
fn test_update_weights_with_insufficient_nonzero() {
    struct TestWeight {
        value: f64,
    }

    impl SampleUniform for TestWeight {
        type Sampler = ();
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight { value: 1.0 }, TestWeight { value: 1.0 }],
        total_weight: TestWeight { value: 2.0 },
        weight_distribution: (),
    };

    let new_weights = vec![(0, &TestWeight { value: 0.0 }), (1, &TestWeight { value: 0.0 })];

    let result = weighted_index.update_weights(&new_weights);
    assert!(matches!(result, Err(Error::InsufficientNonZero)));
}

