// Answer 0

#[test]
fn test_count_left_empty_iterator() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iterator = EmptyIter;
    let either_instance = Either::Left(left_iterator);
    let count = either_instance.count();
}

#[test]
fn test_count_left_single_element_iterator() {
    struct SingleElementIter {
        count: usize,
    }

    impl Iterator for SingleElementIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                self.count += 1;
                Some(42)
            } else {
                None
            }
        }
    }

    let left_iterator = SingleElementIter { count: 0 };
    let either_instance = Either::Left(left_iterator);
    let count = either_instance.count();
}

#[test]
fn test_count_left_multiple_elements_iterator() {
    struct MultipleElementsIter {
        current: usize,
        end: usize,
    }

    impl Iterator for MultipleElementsIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let value = self.current;
                self.current += 1;
                Some(value as i32)
            } else {
                None
            }
        }
    }

    let left_iterator = MultipleElementsIter { current: 0, end: 5 };
    let either_instance = Either::Left(left_iterator);
    let count = either_instance.count();
}

#[test]
fn test_count_left_iterator_with_zero_size() {
    struct ZeroSizeIter;

    impl Iterator for ZeroSizeIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iterator = ZeroSizeIter;
    let either_instance = Either::Left(left_iterator);
    let count = either_instance.count();
}

