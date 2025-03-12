// Answer 0

#[test]
fn test_weighted_index_iter_next_none_out_of_bounds() {
    struct SampleType;
    
    impl SampleUniform for SampleType {
        type Sampler = SampleType;
    }

    let cumulative_weights = vec![1, 2, 3]; // Sample weights
    let total_weight = 6; // Total weight
    let sampler = SampleType;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution: sampler,
    };
    
    let mut iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index: 3, // Index out of bounds since there are only 3 cumulative weights
    };

    let result = iter.next();
}

#[test]
fn test_weighted_index_iter_next_none_exceeding_bounds() {
    struct SampleType;
    
    impl SampleUniform for SampleType {
        type Sampler = SampleType;
    }

    let cumulative_weights = vec![5, 10, 15]; // Sample weights
    let total_weight = 30; // Total weight
    let sampler = SampleType;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution: sampler,
    };
    
    let mut iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index: 4, // Index exceeds the bounds of cumulative_weights
    };

    let result = iter.next();
}

