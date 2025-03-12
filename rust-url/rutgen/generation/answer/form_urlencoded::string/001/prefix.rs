// Answer 0

#[test]
fn test_string_with_none() {
    let mut target: Option<MockTarget> = None;
    string(&mut target);
}

#[test]
fn test_string_with_valid_target() {
    struct MockTarget {
        value: String,
    }

    impl MockTarget {
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }
    }

    let mut target: Option<MockTarget> = Some(MockTarget { value: String::from("valid") });
    string(&mut target);
}

#[test]
#[should_panic(expected = "url::form_urlencoded::Serializer finished")]
fn test_string_with_invalid_target() {
    struct MockTarget {
        value: String,
    }

    impl MockTarget {
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }
    }

    let mut target: Option<MockTarget> = None;
    string(&mut target);
}

#[test]
fn test_string_with_empty_string() {
    struct MockTarget {
        value: String,
    }

    impl MockTarget {
        fn as_mut_string(&mut self) -> &mut String {
            &mut self.value
        }
    }

    let mut target: Option<MockTarget> = Some(MockTarget { value: String::new() });
    string(&mut target);
}

