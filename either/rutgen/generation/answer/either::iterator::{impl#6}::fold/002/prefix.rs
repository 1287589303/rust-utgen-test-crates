// Answer 0

#[test]
fn test_fold_with_left_iterator() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                let value = self.count;
                self.count += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = LeftIter { count: 0 };
    let right_iter = RightIter { count: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iter) };
    
    let result = iter_either.fold(0, |acc, item| {
        match item {
            Either::Left(val) => acc + val,
            Either::Right(_) => acc,
        }
    });
}

#[test]
fn test_fold_with_left_iterator_and_boundary_case() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 1 {
                let value = self.count;
                self.count += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = LeftIter { count: 0 };
    let right_iter = RightIter { count: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iter) };
    
    let result = iter_either.fold(0, |acc, item| {
        match item {
            Either::Left(val) => acc + val,
            Either::Right(_) => acc,
        }
    });
}

