// Answer 0

#[test]
fn test_fold_with_left_iterator() {
    struct LeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iterator = LeftIterator { data: vec![1, 2, 3], index: 0 };
    let either = Either::Left(left_iterator);

    let init_value = 0;
    let sum_function = |acc: i32, x: i32| acc + x;

    let result = either.fold(init_value, sum_function);
}

#[test]
fn test_fold_with_left_empty_iterator() {
    struct LeftEmptyIterator;

    impl Iterator for LeftEmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iterator = LeftEmptyIterator;
    let either = Either::Left(left_iterator);

    let init_value = 10;
    let sum_function = |acc: i32, x: i32| acc + x;

    let result = either.fold(init_value, sum_function);
}

#[test]
fn test_fold_with_single_item_left_iterator() {
    struct SingleItemLeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for SingleItemLeftIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iterator = SingleItemLeftIterator { data: vec![5], index: 0 };
    let either = Either::Left(left_iterator);

    let init_value = 3;
    let sum_function = |acc: i32, x: i32| acc + x;

    let result = either.fold(init_value, sum_function);
}

