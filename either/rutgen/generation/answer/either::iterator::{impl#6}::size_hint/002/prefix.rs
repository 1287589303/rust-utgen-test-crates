// Answer 0

#[test]
fn test_size_hint_left_non_empty() {
    struct LeftIter {
        count: usize,
    }
    
    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(self.count)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    let inner = Either::Left(LeftIter { count: 5 });
    let iter = IterEither { inner };
    let hint = iter.size_hint();
}

#[test]
fn test_size_hint_left_zero_elements_some() {
    struct LeftZeroIter {
        count: usize,
    }
    
    impl Iterator for LeftZeroIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    let inner = Either::Left(LeftZeroIter { count: 0 });
    let iter = IterEither { inner };
    let hint = iter.size_hint();
}

#[test]
fn test_size_hint_left_zero_elements_none() {
    struct LeftNoneIter {
        count: usize,
    }
    
    impl Iterator for LeftNoneIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, None)
        }
    }

    let inner = Either::Left(LeftNoneIter { count: 0 });
    let iter = IterEither { inner };
    let hint = iter.size_hint();
}

