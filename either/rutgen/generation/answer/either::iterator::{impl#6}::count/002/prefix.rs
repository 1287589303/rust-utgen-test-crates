// Answer 0

#[test]
fn test_count_left_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }
    
    let iter_left = Either::Left(EmptyIterator);
    let iter_either = IterEither { inner: iter_left };
    let _ = iter_either.count();
}

#[test]
fn test_count_left_single_element_iterator() {
    struct SingleElementIterator {
        count: usize,
    }

    impl Iterator for SingleElementIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }
    }
    
    let iter_left = Either::Left(SingleElementIterator { count: 1 });
    let iter_either = IterEither { inner: iter_left };
    let _ = iter_either.count();
}

#[test]
fn test_count_left_multiple_elements_iterator() {
    struct MultipleElementsIterator {
        count: usize,
    }

    impl Iterator for MultipleElementsIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }
    }
    
    let iter_left = Either::Left(MultipleElementsIterator { count: 3 });
    let iter_either = IterEither { inner: iter_left };
    let _ = iter_either.count();
}

#[test]
fn test_count_left_large_iterator() {
    struct LargeIterator {
        count: usize,
    }

    impl Iterator for LargeIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }
    }
    
    let iter_left = Either::Left(LargeIterator { count: usize::MAX });
    let iter_either = IterEither { inner: iter_left };
    let _ = iter_either.count();
}

