// Answer 0

#[test]
fn test_choose_multiple_weighted_empty_weights() {
    struct TestStruct {
        data: Vec<usize>,
    }

    impl Index<usize> for TestStruct {
        type Output = usize;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestStruct {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut rng = rand::rngs::OsRng;
    let instance = TestStruct { data: vec![1, 2, 3] };

    let result: Result<_, WeightError> = instance.choose_multiple_weighted(&mut rng, 4, |&item| 0.0);
}

#[test]
fn test_choose_multiple_weighted_insufficient_non_zero_weights() {
    struct TestStruct {
        data: Vec<usize>,
    }

    impl Index<usize> for TestStruct {
        type Output = usize;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestStruct {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut rng = rand::rngs::OsRng;
    let instance = TestStruct { data: vec![5, 5, 5] };

    let result: Result<_, WeightError> = instance.choose_multiple_weighted(&mut rng, 4, |&item| {
        if item == 5 { 0.0 } else { 1.0 }
    });
}

#[test]
fn test_choose_multiple_weighted_high_amount() {
    struct TestStruct {
        data: Vec<usize>,
    }

    impl Index<usize> for TestStruct {
        type Output = usize;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestStruct {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut rng = rand::rngs::OsRng;
    let instance = TestStruct { data: vec![1, 2] };

    let result: Result<_, WeightError> = instance.choose_multiple_weighted(&mut rng, 3, |&item| 1.0);
}

