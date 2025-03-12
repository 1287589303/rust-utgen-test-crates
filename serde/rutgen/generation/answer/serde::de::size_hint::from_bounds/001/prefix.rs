// Answer 0

#[test]
fn test_from_bounds_equal_bounds_zero() {
    struct EqualBoundsIter {
        count: usize,
    }

    impl Iterator for EqualBoundsIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    let iter = EqualBoundsIter { count: 0 };
    let result = from_bounds(&iter);
}

#[test]
fn test_from_bounds_equal_bounds_one() {
    struct EqualBoundsIter {
        count: usize,
    }

    impl Iterator for EqualBoundsIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, Some(1))
        }
    }

    let iter = EqualBoundsIter { count: 1 };
    let result = from_bounds(&iter);
}

#[test]
fn test_from_bounds_equal_bounds_two() {
    struct EqualBoundsIter {
        count: usize,
    }

    impl Iterator for EqualBoundsIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (2, Some(2))
        }
    }

    let iter = EqualBoundsIter { count: 2 };
    let result = from_bounds(&iter);
}

#[test]
fn test_from_bounds_equal_bounds_five() {
    struct EqualBoundsIter {
        count: usize,
    }

    impl Iterator for EqualBoundsIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (5, Some(5))
        }
    }

    let iter = EqualBoundsIter { count: 5 };
    let result = from_bounds(&iter);
}

#[test]
fn test_from_bounds_not_equal_bounds() {
    struct NotEqualBoundsIter {
        count: usize,
    }

    impl Iterator for NotEqualBoundsIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (3, None)
        }
    }

    let iter = NotEqualBoundsIter { count: 3 };
    let result = from_bounds(&iter);
}

#[test]
fn test_from_bounds_zero_none() {
    struct ZeroNoneIter {
        count: usize,
    }

    impl Iterator for ZeroNoneIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, None)
        }
    }

    let iter = ZeroNoneIter { count: 0 };
    let result = from_bounds(&iter);
}

