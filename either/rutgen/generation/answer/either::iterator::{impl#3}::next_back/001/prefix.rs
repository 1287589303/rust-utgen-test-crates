// Answer 0

#[test]
fn test_next_back_with_right_iterator_empty() {
    struct EmptyIterator;

    impl DoubleEndedIterator for EmptyIterator {
        type Item = usize;

        fn next_back(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iterator = Either::Right(EmptyIterator);
    let mut iter = right_iterator;
    iter.next_back();
}

#[test]
fn test_next_back_with_right_iterator_single_element() {
    struct SingleElementIterator {
        has_yielded: bool,
    }

    impl DoubleEndedIterator for SingleElementIterator {
        type Item = usize;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.has_yielded {
                None
            } else {
                self.has_yielded = true;
                Some(1)
            }
        }
    }

    let right_iterator = Either::Right(SingleElementIterator { has_yielded: false });
    let mut iter = right_iterator;
    iter.next_back();
}

#[test]
fn test_next_back_with_right_iterator_multiple_elements() {
    struct MultipleElementIterator {
        elements: Vec<usize>,
        current_index: usize,
    }

    impl DoubleEndedIterator for MultipleElementIterator {
        type Item = usize;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.current_index == 0 {
                None
            } else {
                self.current_index -= 1;
                Some(self.elements[self.current_index])
            }
        }
    }

    let right_iterator = Either::Right(MultipleElementIterator {
        elements: vec![1, 2, 3],
        current_index: 3,
    });
    let mut iter = right_iterator;
    iter.next_back(); // should yield 3
    iter.next_back(); // should yield 2
    iter.next_back(); // should yield 1
    iter.next_back(); // should yield None
}

