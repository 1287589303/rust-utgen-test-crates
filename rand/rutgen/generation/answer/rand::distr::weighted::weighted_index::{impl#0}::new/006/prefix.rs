// Answer 0

#[test]
fn test_weighted_index_invalid_input_empty() {
    let weights: Vec<i32> = vec![];
    let result = WeightedIndex::<i32>::new(weights);
}

#[test]
fn test_weighted_index_invalid_input_zero_weight() {
    let weights = vec![0];
    let result = WeightedIndex::<i32>::new(weights);
}

#[test]
fn test_weighted_index_invalid_input_negative_weight() {
    let weights = vec![-1];
    let result = WeightedIndex::<i32>::new(weights);
}

