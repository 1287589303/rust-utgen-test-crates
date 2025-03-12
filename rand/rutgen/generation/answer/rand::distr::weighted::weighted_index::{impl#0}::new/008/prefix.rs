// Answer 0

#[test]
fn test_weighted_index_new_invalid_weight() {
    struct TestWeight;
    impl Clone for TestWeight {
        fn clone(&self) -> Self {
            TestWeight
        }
    }
    impl Weight for TestWeight {
        const ZERO: Self = TestWeight;

        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Err(())
        }
    }

    let weights = vec![TestWeight, TestWeight]; // Assume some negative weight or NaN is handled here

    let result = WeightedIndex::<TestWeight>::new(weights);
    // The expected output is Err(Error::InvalidWeight)
}

#[test]
fn test_weighted_index_new_negative_weight() {
    struct NegativeWeight;
    impl Clone for NegativeWeight {
        fn clone(&self) -> Self {
            NegativeWeight
        }
    }
    impl Weight for NegativeWeight {
        const ZERO: Self = NegativeWeight;

        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Err(())
        }
    }

    let weights = vec![NegativeWeight]; // Weights are assumed to be negative

    let result = WeightedIndex::<NegativeWeight>::new(weights);
    // The expected output is Err(Error::InvalidWeight)
}

