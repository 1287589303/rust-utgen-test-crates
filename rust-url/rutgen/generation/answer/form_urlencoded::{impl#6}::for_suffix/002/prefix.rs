// Answer 0

#[test]
fn test_for_suffix_start_position_zero() {
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

    let mut target = TestTarget {
        inner: String::from(""),
    };
    let start_position = 0;
    let serializer = Serializer::for_suffix(target, start_position);
}

#[test]
fn test_for_suffix_start_position_equal() {
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

    let mut target = TestTarget {
        inner: String::from("value"),
    };
    let start_position = target.as_mut_string().len();
    let serializer = Serializer::for_suffix(target, start_position);
}

