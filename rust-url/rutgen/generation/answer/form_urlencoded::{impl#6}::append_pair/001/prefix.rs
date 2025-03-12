// Answer 0

#[test]
fn test_append_pair_valid_input() {
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

    let mut target = TestTarget { inner: String::from("initial_value") };
    let mut serializer = Serializer::new(target);
    serializer.append_pair("name", "value");
}

#[test]
fn test_append_pair_empty_strings() {
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

    let mut target = TestTarget { inner: String::from("") };
    let mut serializer = Serializer::new(target);
    serializer.append_pair("", "");
}

#[test]
fn test_append_pair_with_long_strings() {
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

    let mut target = TestTarget { inner: String::from("initial_value") };
    let mut serializer = Serializer::new(target);
    serializer.append_pair("a_long_name_that_exceeds_normally_allowed_length", "a_long_value_that_exceeds_normally_allowed_length");
}

#[test]
fn test_append_pair_with_sufficient_length() {
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

    let mut target = TestTarget { inner: String::from("1234567890") };
    let mut serializer = Serializer::for_suffix(target, 5);
    serializer.append_pair("key", "value");
}

