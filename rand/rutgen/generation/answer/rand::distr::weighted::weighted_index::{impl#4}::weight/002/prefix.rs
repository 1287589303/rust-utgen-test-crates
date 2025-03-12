// Answer 0

#[test]
fn test_weight_equal_index() {
    struct SampleUniformType;
    impl SampleUniform for SampleUniformType {
        type Sampler = ();
    }

    let cumulative_weights = vec![1, 2, 3]; // 3 elements, len is 3
    let total_weight = 6; // sum of weights
    let weight_distribution = ();

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let result = weighted_index.weight(3); // Equal case
    // The expected weight is 6 (total weight)
}

#[test]
fn test_weight_non_zero_index() {
    struct SampleUniformType;
    impl SampleUniform for SampleUniformType {
        type Sampler = ();
    }

    let cumulative_weights = vec![1, 2, 3]; // 3 elements, len is 3
    let total_weight = 6; // sum of weights
    let weight_distribution = ();

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let result = weighted_index.weight(2); // Non-zero index case
    // The expected weight is 3 (3 - 0)
}

