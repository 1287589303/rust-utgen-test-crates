// Answer 0

#[test]
fn test_choose_weighted_mut_empty_slice() {
    struct TestSlice {
        data: Vec<u32>,
    }

    impl TestSlice {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl std::ops::Index<usize> for TestSlice {
        type Output = u32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    let mut slice = TestSlice { data: Vec::new() };
    let mut rng = rand::thread_rng();
    let weight_func = |x: &u32| *x as i32; // Returns invalid weights

    let result = slice.choose_weighted_mut(&mut rng, weight_func);
}

#[test]
fn test_choose_weighted_mut_negative_weight() {
    struct TestSlice {
        data: Vec<i32>,
    }

    impl TestSlice {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl std::ops::Index<usize> for TestSlice {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    let mut slice = TestSlice { data: vec![1, 2, 3] };
    let mut rng = rand::thread_rng();
    let weight_func = |x: &i32| if *x > 1 { -1 } else { 0 }; // Returns negative weights

    let result = slice.choose_weighted_mut(&mut rng, weight_func);
}

#[test]
fn test_choose_weighted_mut_invalid_weight() {
    struct TestSlice {
        data: Vec<f32>,
    }

    impl TestSlice {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl std::ops::Index<usize> for TestSlice {
        type Output = f32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    let mut slice = TestSlice { data: vec![1.0, std::f32::NAN] }; // Uses invalid weight (NaN)
    let mut rng = rand::thread_rng();
    let weight_func = |x: &f32| *x; // Returns invalid weights

    let result = slice.choose_weighted_mut(&mut rng, weight_func);
}

