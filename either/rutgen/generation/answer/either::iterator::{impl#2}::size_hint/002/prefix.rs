// Answer 0

#[test]
fn test_size_hint_left_non_empty() {
    struct LeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                self.index += 1;
                Some(self.data[self.index - 1])
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.data.len() - self.index;
            (len, Some(len))
        }
    }

    let left_iterator = LeftIterator { data: vec![1, 2, 3], index: 0 };
    let either = Either::Left(left_iterator);
    either.size_hint();
}

#[test]
fn test_size_hint_left_empty() {
    struct LeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                self.index += 1;
                Some(self.data[self.index - 1])
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.data.len() - self.index;
            (len, Some(len))
        }
    }

    let left_iterator = LeftIterator { data: vec![], index: 0 };
    let either = Either::Left(left_iterator);
    either.size_hint();
}

