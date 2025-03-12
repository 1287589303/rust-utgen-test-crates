// Answer 0

#[test]
fn test_weighted_index_invalid_input_empty() {
    let weights: Vec<f64> = vec![];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_invalid_weight_nan() {
    let weights: Vec<f64> = vec![f64::NAN];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_invalid_weight_negative() {
    let weights: Vec<f64> = vec![-1.0];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_invalid_weight_zero_negative() {
    let weights: Vec<f64> = vec![0.0, -1.0];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_invalid_weight_zero_nan() {
    let weights: Vec<f64> = vec![0.0, f64::NAN];
    let result = WeightedIndex::new(weights);
}

