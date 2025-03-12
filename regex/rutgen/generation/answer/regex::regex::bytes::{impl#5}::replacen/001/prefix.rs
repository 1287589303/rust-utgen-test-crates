// Answer 0

#[test]
fn test_replacen_empty_haystack_limit_zero() {
    let re = Regex::new(r"pattern-that-does-not-match").unwrap();
    let haystack: &[u8] = b"";
    let limit = 0;
    struct NoExpansionReplacer;

    impl Replacer for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }
    }

    let rep = NoExpansionReplacer;
    let result = re.replacen(haystack, limit, rep);
}

#[test]
fn test_replacen_empty_haystack_limit_zero_with_non_matching_pattern() {
    let re = Regex::new(r"another-pattern-that-will-not-match").unwrap();
    let haystack: &[u8] = b"";
    let limit = 0;
    struct NoExpansionReplacer;

    impl Replacer for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }
    }

    let rep = NoExpansionReplacer;
    let result = re.replacen(haystack, limit, rep);
}

