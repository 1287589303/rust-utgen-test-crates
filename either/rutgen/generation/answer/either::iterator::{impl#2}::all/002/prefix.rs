// Answer 0

#[test]
fn test_all_with_both_iterators_returning_true() {
    struct TrueIterator {
        count: usize,
    }

    impl Iterator for TrueIterator {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(1)  // All values return true
            } else {
                None
            }
        }
    }

    struct TrueIteratorR {
        count: usize,
    }

    impl Iterator for TrueIteratorR {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(1)
            } else {
                None
            }
        }
    }

    let left = TrueIterator { count: 0 };
    let right = TrueIteratorR { count: 0 };
    let either = Either::Left(left).right(right);
    either.all(|x| x == 1);
}

#[test]
fn test_all_with_mixed_iterators_returning_true_and_false() {
    struct MixedIterator {
        count: usize,
    }

    impl Iterator for MixedIterator {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 6 {
                let value = if self.count % 2 == 0 { 1 } else { 0 };
                self.count += 1;
                Some(value)  // Alternates between true and false
            } else {
                None
            }
        }
    }

    struct MixedIteratorR {
        count: usize,
    }

    impl Iterator for MixedIteratorR {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 6 {
                let value = if self.count % 2 == 0 { 1 } else { 0 };
                self.count += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left = MixedIterator { count: 0 };
    let right = MixedIteratorR { count: 0 };
    let either = Either::Left(left).right(right);
    either.all(|x| x == 1);
}

#[test]
fn test_all_with_empty_iterators() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None  // No elements to return
        }
    }

    struct EmptyIteratorR;

    impl Iterator for EmptyIteratorR {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left = EmptyIterator;
    let right = EmptyIteratorR;
    let either = Either::Left(left).right(right);
    either.all(|x| x == 1);
}

#[test]
fn test_all_with_single_element_returning_true() {
    struct SingleTrueIterator;

    impl Iterator for SingleTrueIterator {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            Some(1)  // Returns true
        }
    }

    struct SingleTrueIteratorR;

    impl Iterator for SingleTrueIteratorR {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None  // No more elements
        }
    }

    let left = SingleTrueIterator;
    let right = SingleTrueIteratorR;
    let either = Either::Left(left).right(right);
    either.all(|x| x == 1);
}

#[test]
fn test_all_with_single_element_returning_false() {
    struct SingleFalseIterator;

    impl Iterator for SingleFalseIterator {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            Some(0)  // Returns false
        }
    }

    struct SingleFalseIteratorR;

    impl Iterator for SingleFalseIteratorR {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None  // No more elements
        }
    }

    let left = SingleFalseIterator;
    let right = SingleFalseIteratorR;
    let either = Either::Left(left).right(right);
    either.all(|x| x == 1);
}

