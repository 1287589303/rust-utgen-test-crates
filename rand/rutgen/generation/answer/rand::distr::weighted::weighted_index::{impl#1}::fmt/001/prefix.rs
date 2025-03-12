// Answer 0

#[test]
fn test_weighted_index_iter_debug_empty() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType;
    }
    
    let cumulative_weights: Vec<SampleType> = Vec::new();
    let total_weight: SampleType = SampleType;
    let weight_distribution = SampleType;
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };
    
    let index = 0; // valid for empty weights
    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index,
    };
    
    let _ = fmt::write(format!("/fmt/debug_struct: {:?}", iter));
}

#[test]
fn test_weighted_index_iter_debug_small() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType;
    }

    let cumulative_weights = vec![SampleType, SampleType];
    let total_weight: SampleType = SampleType;
    let weight_distribution = SampleType;
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    for index in 0..weighted_index.cumulative_weights.len() {
        let iter = WeightedIndexIter {
            weighted_index: &weighted_index,
            index,
        };
        let _ = fmt::write(format!("/fmt/debug_struct: {:?}", iter));
    }
}

#[test]
fn test_weighted_index_iter_debug_large() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType;
    }

    let cumulative_weights: Vec<SampleType> = vec![SampleType; 100]; // large size
    let total_weight: SampleType = SampleType;
    let weight_distribution = SampleType;
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    for index in 0..weighted_index.cumulative_weights.len() {
        let iter = WeightedIndexIter {
            weighted_index: &weighted_index,
            index,
        };
        let _ = fmt::write(format!("/fmt/debug_struct: {:?}", iter));
    }
}

#[test]
fn test_weighted_index_iter_debug_boundary() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType;
    }

    let cumulative_weights = vec![SampleType]; // single element
    let total_weight: SampleType = SampleType;
    let weight_distribution = SampleType;
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let index = weighted_index.cumulative_weights.len(); // boundary case, should be valid if len() == 1
    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index,
    };
    let _ = fmt::write(format!("/fmt/debug_struct: {:?}", iter));
}

