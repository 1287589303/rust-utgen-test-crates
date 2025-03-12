// Answer 0

#[test]
fn test_replacen_empty_haystack_limit_zero() {
    struct NoExpandingReplacer {
        replacement: &'static str,
    }

    impl NoExpandingReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.replacement)
        }
    }

    let re = Regex::new(r"pattern").unwrap();
    let haystack = "";
    let limit = 0;
    let rep = NoExpandingReplacer { replacement: "replacement_string" };

    let _result = re.replacen(haystack, limit, rep);
}

#[test]
fn test_replacen_empty_haystack_limit_zero_with_other_replacement() {
    struct NoExpandingReplacer {
        replacement: &'static str,
    }

    impl NoExpandingReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.replacement)
        }
    }

    let re = Regex::new(r"pattern").unwrap();
    let haystack = "";
    let limit = 0;
    let rep = NoExpandingReplacer { replacement: "another_replacement" };

    let _result = re.replacen(haystack, limit, rep);
}

