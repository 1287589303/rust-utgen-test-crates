// Answer 0

#[test]
fn test_finish_empty_target() {
    struct TestTarget {
        data: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.data
        }
        fn finish(self) -> Self::Finished {
            self.data
        }
    }

    let encoded: String = Serializer::new(TestTarget { data: String::new() })
        .finish();
}

#[test]
fn test_finish_single_pair() {
    struct TestTarget {
        data: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.data
        }
        fn finish(self) -> Self::Finished {
            self.data
        }
    }

    let encoded: String = Serializer::new(TestTarget { data: String::new() })
        .append_pair("foo", "bar & baz")
        .finish();
}

#[test]
fn test_finish_special_characters() {
    struct TestTarget {
        data: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.data
        }
        fn finish(self) -> Self::Finished {
            self.data
        }
    }

    let encoded: String = Serializer::new(TestTarget { data: String::new() })
        .append_pair("saison", "Été+hiver")
        .finish();
}

#[test]
fn test_finish_multiple_pairs() {
    struct TestTarget {
        data: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.data
        }
        fn finish(self) -> Self::Finished {
            self.data
        }
    }

    let encoded: String = Serializer::new(TestTarget { data: String::new() })
        .append_pair("foo", "bar & baz")
        .append_pair("saison", "Été+hiver")
        .finish();
}

#[test]
#[should_panic]
fn test_finish_panic_double_finish() {
    struct TestTarget {
        data: String,
    }

    impl Target for TestTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.data
        }
        fn finish(self) -> Self::Finished {
            self.data
        }
    }

    let mut serializer = Serializer::new(TestTarget { data: String::new() });
    serializer.finish();
    serializer.finish(); // This should cause a panic
}

