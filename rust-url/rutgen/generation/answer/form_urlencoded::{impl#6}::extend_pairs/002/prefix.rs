// Answer 0

#[test]
fn test_extend_pairs_empty_iterable() {
    struct TargetMock {
        value: String,
    }
    
    impl Target for TargetMock {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }

        fn finish(self) -> Self::Finished {
            self.value
        }
    }

    let mut target = TargetMock { value: String::from("initial") };
    let mut serializer = Serializer::new(target);
    let empty_iter: Vec<(&str, &str)> = Vec::new();
    serializer.extend_pairs(empty_iter);
}

#[test]
fn test_extend_pairs_with_empty_strings() {
    struct TargetMock {
        value: String,
    }

    impl Target for TargetMock {
        type Finished = String;

        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }

        fn finish(self) -> Self::Finished {
            self.value
        }
    }

    let mut target = TargetMock { value: String::from("initial") };
    let mut serializer = Serializer::new(target);
    let empty_pairs: Vec<(&str, &str)> = vec![("", "")];
    serializer.extend_pairs(empty_pairs);
}

