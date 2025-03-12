// Answer 0

#[test]
fn test_extend_keys_only_with_empty_iterable() {
    struct TestTarget {
        string: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.string
        }

        fn finish(self) -> Self::Finished {
            self.string
        }
    }

    let mut target = TestTarget {
        string: String::from("initial"),
    };

    let mut serializer = Serializer::for_suffix(target, 0);
    serializer.extend_keys_only(vec![]);
}

#[test]
fn test_extend_keys_only_with_no_borrow() {
    struct TestTarget {
        string: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.string
        }

        fn finish(self) -> Self::Finished {
            self.string
        }
    }

    let mut target = TestTarget {
        string: String::from("initial"),
    };

    let mut serializer = Serializer::for_suffix(target, 0);
    serializer.extend_keys_only(vec!["key1", "key2"]);
}

#[test]
#[should_panic]
fn test_extend_keys_only_after_finish() {
    struct TestTarget {
        string: String,
    }

    impl Target for TestTarget {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.string
        }

        fn finish(self) -> Self::Finished {
            self.string
        }
    }

    let mut target = TestTarget {
        string: String::from("initial"),
    };

    let mut serializer = Serializer::for_suffix(target, 0);
    serializer.finish();
    serializer.extend_keys_only(vec!["key1"]);
}

