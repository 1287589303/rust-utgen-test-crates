// Answer 0

#[test]
fn test_next_with_valid_index_zero() {
    struct SampleValue;
    impl SampleUniform for SampleValue {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![SampleValue];
    let total_weight = SampleValue;
    let weight_distribution = ();
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };
    
    let mut iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index: 0,
    };

    iter.next();
}

#[test]
fn test_next_with_valid_index_mid() {
    struct SampleValue;
    impl SampleUniform for SampleValue {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![SampleValue, SampleValue, SampleValue];
    let total_weight = SampleValue;
    let weight_distribution = ();
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };
    
    let mut iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index: 1,
    };

    iter.next();
}

#[test]
fn test_next_with_valid_index_end() {
    struct SampleValue;
    impl SampleUniform for SampleValue {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![SampleValue, SampleValue, SampleValue];
    let total_weight = SampleValue;
    let weight_distribution = ();
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };
    
    let mut iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index: 2,
    };

    iter.next();
}

