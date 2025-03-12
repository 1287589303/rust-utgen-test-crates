// Answer 0

#[test]
fn test_replacen_with_non_empty_haystack_and_matches() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement_string")
        }

        fn replace_append(&self, caps: &Captures, dst: &mut String) {
            if let Some(m) = caps.get(0) {
                dst.push_str("replacement_string");
            }
        }
    }

    let regex = Regex::new(r"foo").unwrap();
    let haystack = "foo bar foo baz";
    let limit = 1;
    let rep = MockReplacer;

    let result = regex.replacen(haystack, limit, rep);
}

#[test]
fn test_replacen_with_limit_and_multiple_matches() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement_string")
        }

        fn replace_append(&self, caps: &Captures, dst: &mut String) {
            if let Some(m) = caps.get(0) {
                dst.push_str("replacement_string");
            }
        }
    }

    let regex = Regex::new(r"bar").unwrap();
    let haystack = "foo bar foo bar baz";
    let limit = 2;
    let rep = MockReplacer;

    let result = regex.replacen(haystack, limit, rep);
}

#[test]
fn test_replacen_with_limit_equal_to_matches() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement_string")
        }

        fn replace_append(&self, caps: &Captures, dst: &mut String) {
            if let Some(m) = caps.get(0) {
                dst.push_str("replacement_string");
            }
        }
    }

    let regex = Regex::new(r"foo").unwrap();
    let haystack = "foo bar foo baz foo";
    let limit = 2;
    let rep = MockReplacer;

    let result = regex.replacen(haystack, limit, rep);
}

