// Answer 0

#[test]
fn test_last_with_single_element_left_iterator() {
    struct SingleElementIterator {
        value: Option<i32>,
    }

    impl Iterator for SingleElementIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.value.take()
        }
    }

    let left_iter = SingleElementIterator { value: Some(42) };
    let right_iter = iter::empty::<i32>();
    let either_instance = Either::Left(left_iter);
    let result = either_instance.last();
}

#[test]
fn test_last_with_multiple_elements_left_iterator() {
    struct MultipleElementIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for MultipleElementIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iter = MultipleElementIterator {
        values: vec![1, 2, 3],
        index: 0,
    };
    let right_iter = iter::empty::<i32>();
    let either_instance = Either::Left(left_iter);
    let result = either_instance.last();
}

#[test]
fn test_last_with_empty_right_iterator() {
    struct SingleElementIterator {
        value: Option<i32>,
    }

    impl Iterator for SingleElementIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.value.take()
        }
    }

    let left_iter = SingleElementIterator { value: Some(99) };
    let right_iter = iter::empty::<i32>();
    let either_instance = Either::Left(left_iter);
    let result = either_instance.last();
}

#[test]
fn test_last_with_non_empty_right_iterator() {
    struct SingleElementIterator {
        value: Option<i32>,
    }

    impl Iterator for SingleElementIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.value.take()
        }
    }

    let left_iter = SingleElementIterator { value: Some(17) };
    let right_iter = std::iter::once(25); // Right has one element
    let either_instance = Either::Left(left_iter);
    let result = either_instance.last();
}

