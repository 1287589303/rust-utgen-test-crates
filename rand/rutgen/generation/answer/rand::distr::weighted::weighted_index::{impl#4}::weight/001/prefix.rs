// Answer 0

#[test]
fn test_weight_index_out_of_bounds_greater() {
    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }

    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let dist = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution: TestSampler,
    };

    let out_of_bounds_index = dist.cumulative_weights.len() + 1; // index greater than cumulative_weights.len()
    let result = dist.weight(out_of_bounds_index);
    assert_eq!(result, None);
}

#[test]
fn test_weight_index_out_of_bounds_equal() {
    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }

    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let dist = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution: TestSampler,
    };

    let out_of_bounds_index = dist.cumulative_weights.len(); // index equal to cumulative_weights.len()
    let result = dist.weight(out_of_bounds_index);
    assert_eq!(result, None);
}

