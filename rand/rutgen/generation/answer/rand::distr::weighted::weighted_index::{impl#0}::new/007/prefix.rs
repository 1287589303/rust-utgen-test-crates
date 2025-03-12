// Answer 0

#[test]
fn test_weighted_index_new_valid_case() {
    struct TestWeight;

    impl Clone for TestWeight {
        fn clone(&self) -> Self {
            TestWeight
        }
    }

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight;

        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Ok(())
        }
    }

    let weights = vec![TestWeight, TestWeight];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_new_total_weight_non_zero() {
    struct TestWeight;

    impl Clone for TestWeight {
        fn clone(&self) -> Self {
            TestWeight
        }
    }

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight;

        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Ok(())
        }
    }

    let weights = vec![TestWeight, TestWeight]; // Total used will not be zero
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_new_non_negative_weights() {
    struct TestWeight;

    impl Clone for TestWeight {
        fn clone(&self) -> Self {
            TestWeight
        }
    }

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight;

        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Ok(())
        }
    }

    let weights = vec![TestWeight, TestWeight]; // All weights are non-negative
    let result = WeightedIndex::new(weights);
}

