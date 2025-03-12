// Answer 0

#[test]
fn test_len_empty_iterator() {
    use std::collections::hash_map::RandomState;

    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let hasher = RandomState::new();
    let empty_iter = EmptyIter;
    let splice = Splice { iter: crate::map::Splice::new(empty_iter, &hasher) };
    let result = splice.len();
}

#[test]
fn test_len_single_element_iterator() {
    use std::collections::hash_map::RandomState;

    struct SingleElementIter {
        has_next: bool,
    }

    impl Iterator for SingleElementIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.has_next {
                self.has_next = false;
                Some(42)
            } else {
                None
            }
        }
    }

    let hasher = RandomState::new();
    let single_iter = SingleElementIter { has_next: true };
    let splice = Splice { iter: crate::map::Splice::new(single_iter, &hasher) };
    let result = splice.len();
}

#[test]
fn test_len_multiple_elements_iterator() {
    use std::collections::hash_map::RandomState;

    struct MultipleElementsIter {
        count: usize,
    }

    impl Iterator for MultipleElementsIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(5)
            } else {
                None
            }
        }
    }

    let hasher = RandomState::new();
    let multiple_iter = MultipleElementsIter { count: 10 };
    let splice = Splice { iter: crate::map::Splice::new(multiple_iter, &hasher) };
    let result = splice.len();
}

#[test]
fn test_len_maximum_elements_iterator() {
    use std::collections::hash_map::RandomState;

    struct MaxElementsIter {
        count: usize,
    }

    impl Iterator for MaxElementsIter {
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

    let hasher = RandomState::new();
    let max_count = usize::MAX;
    let max_iter = MaxElementsIter { count: max_count };
    let splice = Splice { iter: crate::map::Splice::new(max_iter, &hasher) };
    let result = splice.len();
}

