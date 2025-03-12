// Answer 0

#[test]
#[should_panic]
fn test_for_suffix_panic_case_1() {
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
        string: String::from("val=value"),
    };
    let start_position = target.as_mut_string().len() + 1; // This should trigger the panic

    Serializer::for_suffix(target, start_position);
}

#[test]
#[should_panic]
fn test_for_suffix_panic_case_2() {
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
        string: String::from("key=value"),
    };
    let start_position = target.as_mut_string().len() + 1; // This should trigger the panic

    Serializer::for_suffix(target, start_position);
}

