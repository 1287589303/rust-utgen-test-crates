// Answer 0

#[test]
fn test_weighted_index_new_invalid_input() {
    let weights: Vec<f32> = Vec::new(); // empty iterator
    let result: Result<WeightedIndex<f32>, Error> = WeightedIndex::new(weights);
    // No assertions, only calling the function
}

#[test]
fn test_weighted_index_new_invalid_weight_negative() {
    let weights = vec![1.0, -1.0]; // second weight is negative
    let result: Result<WeightedIndex<f32>, Error> = WeightedIndex::new(weights);
    // No assertions, only calling the function
}

#[test]
fn test_weighted_index_new_insufficient_non_zero() {
    let weights = vec![0.0, 0.0]; // total weight is zero
    let result: Result<WeightedIndex<f32>, Error> = WeightedIndex::new(weights);
    // No assertions, only calling the function
}

#[test]
fn test_weighted_index_new_overflow() {
    #[derive(Debug, Clone, PartialOrd)]
    struct CustomWeight(u32);
    
    impl Weight for CustomWeight {
        const ZERO: Self = CustomWeight(0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0.checked_add(v.0);
            match new_value {
                Some(_) => {
                    self.0 = new_value.unwrap();
                    Ok(())
                },
                None => Err(()),
            }
        }
    }
    
    let weights = vec![CustomWeight(u32::MAX), CustomWeight(1)]; // will overflow
    let result: Result<WeightedIndex<CustomWeight>, Error> = WeightedIndex::new(weights);
    // No assertions, only calling the function
}

