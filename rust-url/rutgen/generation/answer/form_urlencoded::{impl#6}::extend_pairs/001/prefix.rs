// Answer 0

#[test]
fn test_extend_pairs_with_valid_pairs() {
    struct TestTarget {
        inner: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.inner
        }

        fn finish(self) -> Self::Finished {
            self.inner
        }
    }

    let mut target = TestTarget { inner: String::new() };
    let mut serializer = Serializer::new(target);
    let valid_pairs = vec![
        ("key1", "value1"),
        ("key2", "value2"),
    ];
    serializer.extend_pairs(valid_pairs.iter());
}

#[test]
fn test_extend_pairs_with_empty_iter() {
    struct TestTarget {
        inner: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.inner
        }

        fn finish(self) -> Self::Finished {
            self.inner
        }
    }

    let mut target = TestTarget { inner: String::new() };
    let mut serializer = Serializer::new(target);
    let empty_pairs: Vec<(&str, &str)> = Vec::new();
    serializer.extend_pairs(empty_pairs.iter());
}

#[test]
fn test_extend_pairs_with_empty_key_value() {
    struct TestTarget {
        inner: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.inner
        }

        fn finish(self) -> Self::Finished {
            self.inner
        }
    }

    let mut target = TestTarget { inner: String::new() };
    let mut serializer = Serializer::new(target);
    let invalid_pairs = vec![
        ("", "value"),
        ("key", ""),
    ];
    serializer.extend_pairs(invalid_pairs.iter());
}

#[test]
#[should_panic]
fn test_extend_pairs_panic_after_finish() {
    struct TestTarget {
        inner: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.inner
        }

        fn finish(self) -> Self::Finished {
            self.inner
        }
    }

    let mut target = TestTarget { inner: String::new() };
    let mut serializer = Serializer::new(target);
    let valid_pairs = vec![("key1", "value1")];
    serializer.extend_pairs(valid_pairs.iter());
    serializer.finish();
    serializer.extend_pairs(valid_pairs.iter());
}

#[test]
fn test_extend_pairs_boundary_length() {
    struct TestTarget {
        inner: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.inner
        }

        fn finish(self) -> Self::Finished {
            self.inner
        }
    }

    let mut target = TestTarget { inner: String::new() };
    let mut serializer = Serializer::new(target);
    let boundary_pairs = vec![
        ("a", "b"),
        ("key", "value with length of maximum string"),
    ];
    serializer.extend_pairs(boundary_pairs.iter());
}

