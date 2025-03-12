// Answer 0

#[test]
fn test_clear_with_valid_target_and_start_position() {
    struct TestTarget {
        value: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }
        fn finish(self) -> Self::Finished {
            self.value
        }
    }

    let mut target = TestTarget {
        value: String::from("HelloWorld"),
    };
    let mut serializer = Serializer::for_suffix(target, 5);
    serializer.clear();
}

#[test]
#[should_panic]
fn test_clear_with_start_position_exceeding_target_length() {
    struct TestTarget {
        value: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }
        fn finish(self) -> Self::Finished {
            self.value
        }
    }

    let mut target = TestTarget {
        value: String::from("Short"),
    };
    let mut serializer = Serializer::for_suffix(target, 10);
    serializer.clear();
}

#[test]
fn test_clear_at_zero_start_position() {
    struct TestTarget {
        value: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }
        fn finish(self) -> Self::Finished {
            self.value
        }
    }

    let mut target = TestTarget {
        value: String::from("Sample"),
    };
    let mut serializer = Serializer::for_suffix(target, 0);
    serializer.clear();
}

