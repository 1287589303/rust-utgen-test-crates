// Answer 0

#[test]
fn test_size_hint_lower_less_than_upper() {
    struct TestMapDeserializer {
        iter: TestIter,
    }

    struct TestIter {
        lower: usize,
        upper: usize,
    }

    impl TestIter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.lower, Some(self.upper))
        }
    }

    let deserializer = TestMapDeserializer {
        iter: TestIter { lower: 1, upper: 2 },
    };

    let result = deserializer.size_hint();
}

#[test]
fn test_size_hint_lower_less_than_upper_zero_one() {
    struct TestMapDeserializer {
        iter: TestIter,
    }

    struct TestIter {
        lower: usize,
        upper: usize,
    }

    impl TestIter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.lower, Some(self.upper))
        }
    }

    let deserializer = TestMapDeserializer {
        iter: TestIter { lower: 0, upper: 1 },
    };

    let result = deserializer.size_hint();
}

