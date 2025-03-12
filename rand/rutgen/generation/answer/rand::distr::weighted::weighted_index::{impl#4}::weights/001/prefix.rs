// Answer 0

#[test]
fn test_weights_with_valid_integers() {
    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let weights = vec![1, 2, 3];
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight: 6,
        weight_distribution: TestSampler,
    };
    
    let iter = dist.weights();
}

#[test]
fn test_weights_with_single_integer() {
    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let weights = vec![4];
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight: 4,
        weight_distribution: TestSampler,
    };

    let iter = dist.weights();
}

#[test]
fn test_weights_with_floats() {
    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let weights = vec![1.5, 2.5, 3.5];
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight: 7.5,
        weight_distribution: TestSampler,
    };

    let iter = dist.weights();
}

#[test]
fn test_weights_with_large_numbers() {
    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let weights = vec![100_000, 200_000, 300_000];
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight: 600_000,
        weight_distribution: TestSampler,
    };

    let iter = dist.weights();
}

#[test]
fn test_weights_with_zero_weight() {
    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let weights = vec![0.0, 1.0, 2.0];
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight: 3.0,
        weight_distribution: TestSampler,
    };

    let iter = dist.weights();
}

