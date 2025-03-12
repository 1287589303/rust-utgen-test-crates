// Answer 0

#[test]
fn test_new_with_non_empty_target() {
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

    let valid_target = TestTarget { inner: String::from("valid") };
    let _serializer = Serializer::new(valid_target);
}

#[test]
fn test_new_with_empty_target() {
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

    let empty_target = TestTarget { inner: String::from("") };
    let _serializer = Serializer::new(empty_target);
}

#[test]
#[should_panic]
fn test_new_with_target_length_greater_than_0() {
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

    let invalid_target = TestTarget { inner: String::from("1234567890") };
    let _serializer = Serializer::for_suffix(invalid_target, 1);
}

#[test]
fn test_new_with_special_characters() {
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

    let special_target = TestTarget { inner: String::from("!@#$%^&*()") };
    let _serializer = Serializer::new(special_target);
}

