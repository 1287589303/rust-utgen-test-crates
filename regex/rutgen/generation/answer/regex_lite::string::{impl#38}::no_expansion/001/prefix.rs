// Answer 0

#[test]
fn test_no_expansion_some() {
    struct TestReplacer {
        expansion: Option<Cow<'static, str>>,
    }

    impl Replacer for TestReplacer {
        fn replace_append(&mut self, _caps: &Captures<'_>, _dst: &mut String) {}
        
        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            self.expansion.clone()
        }
    }

    let mut replacer = TestReplacer { expansion: Some(Cow::Borrowed("test expansion")) };
    let mut replacer_ref = replacer.by_ref();
    let result = replacer_ref.no_expansion();
}

#[test]
fn test_no_expansion_none() {
    struct TestReplacerNone;

    impl Replacer for TestReplacerNone {
        fn replace_append(&mut self, _caps: &Captures<'_>, _dst: &mut String) {}

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            None
        }
    }

    let mut replacer_none = TestReplacerNone;
    let mut replacer_ref_none = replacer_none.by_ref();
    let result_none = replacer_ref_none.no_expansion();
}

#[test]
fn test_no_expansion_empty() {
    struct TestReplacerEmpty {
        expansion: Option<Cow<'static, str>>,
    }

    impl Replacer for TestReplacerEmpty {
        fn replace_append(&mut self, _caps: &Captures<'_>, _dst: &mut String) {}
        
        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            if self.expansion.is_none() {
                Some(Cow::Owned(String::new()))
            } else {
                self.expansion.clone()
            }
        }
    }

    let mut replacer_empty = TestReplacerEmpty { expansion: None };
    let mut replacer_ref_empty = replacer_empty.by_ref();
    let result_empty = replacer_ref_empty.no_expansion();
}

