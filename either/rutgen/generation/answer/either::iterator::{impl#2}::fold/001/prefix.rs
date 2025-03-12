// Answer 0

#[test]
fn test_fold_with_right_variant() {
    struct LeftIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct RightIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { values: vec![1, 2, 3], index: 0 };
    let right_iter = RightIterator { values: vec![4, 5, 6], index: 0 };
    let either_instance = Either::Right(right_iter);

    let init_value = 0;
    let sum_function = |acc: i32, item: i32| acc + item;

    let _result = either_instance.fold(init_value, sum_function);
}

#[test]
fn test_fold_with_different_acc_type() {
    struct RightIterator {
        values: Vec<String>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index].clone();
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator { values: vec!["a".to_string(), "b".to_string()], index: 0 };
    let either_instance = Either::Right(right_iter);
    
    let init_value = String::new();
    let concat_function = |acc: String, item: String| acc + &item;

    let _result = either_instance.fold(init_value, concat_function);
}

#[test]
fn test_fold_with_empty_iterator() {
    struct RightIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator { values: vec![], index: 0 };
    let either_instance = Either::Right(right_iter);

    let init_value = 10;
    let sum_function = |acc: i32, item: i32| acc + item;

    let _result = either_instance.fold(init_value, sum_function);
}

