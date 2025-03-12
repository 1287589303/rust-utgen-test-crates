// Answer 0

#[test]
fn test_no_expansion_none() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn replace_append(&mut self, _: &Captures<'_>, _: &mut String) {}
    }

    let mut replacer = TestReplacer;
    let replacer_ref = ReplacerRef(&mut replacer);
    replacer_ref.no_expansion();
}

#[test]
fn test_no_expansion_some() {
    struct TestReplacerWithSome;

    impl Replacer for TestReplacerWithSome {
        fn replace_append(&mut self, _: &Captures<'_>, _: &mut String) {}
        
        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            Some(Cow::Borrowed("some_value"))
        }
    }

    let mut replacer = TestReplacerWithSome;
    let replacer_ref = ReplacerRef(&mut replacer);
    replacer_ref.no_expansion();
}

