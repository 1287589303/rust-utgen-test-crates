// Answer 0

#[test]
fn test_len_with_non_empty_right() {
    struct LeftIter {
        data: Vec<i32>,
    }

    impl ExactSizeIterator for LeftIter {
        type Item = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    struct RightIter {
        data: Vec<i32>,
    }

    impl ExactSizeIterator for RightIter {
        type Item = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    let left = LeftIter { data: vec![1, 2, 3] };
    let right = RightIter { data: vec![4, 5] };
    let either = Either::Right(right);

    let _result = either.len();
}

#[test]
fn test_len_with_non_empty_left() {
    struct LeftIter {
        data: Vec<i32>,
    }

    impl ExactSizeIterator for LeftIter {
        type Item = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    struct RightIter {
        data: Vec<i32>,
    }

    impl ExactSizeIterator for RightIter {
        type Item = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    let left = LeftIter { data: vec![1, 2, 3] };
    let right = RightIter { data: vec![4] };
    let either = Either::Right(right);

    let _result = either.len();
}

#[test]
fn test_len_with_single_element_right() {
    struct LeftIter {
        data: Vec<i32>,
    }

    impl ExactSizeIterator for LeftIter {
        type Item = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    struct RightIter {
        data: Vec<i32>,
    }

    impl ExactSizeIterator for RightIter {
        type Item = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    let left = LeftIter { data: vec![1] };
    let right = RightIter { data: vec![2] };
    let either = Either::Right(right);
    
    let _result = either.len();
}

