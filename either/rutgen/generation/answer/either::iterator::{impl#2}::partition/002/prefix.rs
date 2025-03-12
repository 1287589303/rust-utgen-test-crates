// Answer 0

#[test]
fn test_partition_with_non_empty_left_and_empty_right() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct RightIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None // Empty iterator
        }
    }

    let left_iter = LeftIter { data: vec![1, 2, 3], index: 0 };
    let right_iter = RightIter { data: vec![], index: 0 };

    let either = Either::Left(left_iter);
    let predicate = |&x: &i32| x % 2 == 0;

    let (_even_items, _odd_items): (Vec<i32>, Vec<i32>) = either.partition(predicate);
}

#[test]
fn test_partition_with_non_empty_left_and_non_empty_right() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct RightIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { data: vec![1, 2, 3], index: 0 };
    let right_iter = RightIter { data: vec![4, 5, 6], index: 0 };

    let either = Either::Left(left_iter);
    let predicate = |&x: &i32| x % 2 == 0;

    let (_even_items, _odd_items): (Vec<i32>, Vec<i32>) = either.partition(predicate);
}

