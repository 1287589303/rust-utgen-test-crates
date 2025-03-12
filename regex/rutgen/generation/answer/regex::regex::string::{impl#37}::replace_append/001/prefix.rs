// Answer 0

#[test]
fn test_replace_append_valid_input() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
            dst.push_str(caps.haystack);
        }
    }

    let mut replacer = TestReplacer;
    let haystack = "sample text";
    let caps = Captures {
        haystack,
        caps: captures::Captures::default(),
        static_captures_len: None,
    };
    let mut dst = String::new();

    let mut replacer_ref = replacer.by_ref();
    replacer_ref.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_empty_dst() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
            dst.push_str(caps.haystack);
        }
    }

    let mut replacer = TestReplacer;
    let haystack = "empty destination";
    let caps = Captures {
        haystack,
        caps: captures::Captures::default(),
        static_captures_len: None,
    };
    let mut dst = String::new();

    let mut replacer_ref = replacer.by_ref();
    replacer_ref.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_long_haystack() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
            dst.push_str(caps.haystack);
        }
    }

    let mut replacer = TestReplacer;
    let haystack = "This is a considerably longer test string.";
    let caps = Captures {
        haystack,
        caps: captures::Captures::default(),
        static_captures_len: None,
    };
    let mut dst = String::new();

    let mut replacer_ref = replacer.by_ref();
    replacer_ref.replace_append(&caps, &mut dst);
}

