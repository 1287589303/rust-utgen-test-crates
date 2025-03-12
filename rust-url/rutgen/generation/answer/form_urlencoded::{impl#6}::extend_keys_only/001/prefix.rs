// Answer 0

#[test]
fn test_extend_keys_only_valid_input() {
    struct MockTarget {
        string: String,
    }
    
    impl Target for MockTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.string
        }
        fn finish(self) -> Self::Finished {
            self.string
        }
    }

    let mut target = MockTarget { string: String::new() };
    let mut serializer = Serializer::new(target);
    
    let keys = vec!["key1", "key2", "key3"];
    serializer.extend_keys_only(keys.iter());
}

#[test]
fn test_extend_keys_only_empty_iterator() {
    struct MockTarget {
        string: String,
    }
    
    impl Target for MockTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.string
        }
        fn finish(self) -> Self::Finished {
            self.string
        }
    }

    let mut target = MockTarget { string: String::new() };
    let mut serializer = Serializer::new(target);
    
    let keys: Vec<&str> = Vec::new();
    serializer.extend_keys_only(keys.iter());
}

#[test]
fn test_extend_keys_only_null_or_empty_strings() {
    struct MockTarget {
        string: String,
    }
    
    impl Target for MockTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.string
        }
        fn finish(self) -> Self::Finished {
            self.string
        }
    }

    let mut target = MockTarget { string: String::new() };
    let mut serializer = Serializer::new(target);
    
    let keys = vec!["", "key", null];
    serializer.extend_keys_only(keys.iter());
}

#[test]
fn test_extend_keys_only_special_characters() {
    struct MockTarget {
        string: String,
    }
    
    impl Target for MockTarget {
        type Finished = String;
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.string
        }
        fn finish(self) -> Self::Finished {
            self.string
        }
    }

    let mut target = MockTarget { string: String::new() };
    let mut serializer = Serializer::new(target);
    
    let keys = vec!["@special", "#tag", "!important"];
    serializer.extend_keys_only(keys.iter());
}

