// Answer 0

#[test]
fn test_replacer_by_ref_with_mutable_instance() {
    struct SimpleReplacer;

    impl Replacer for SimpleReplacer {
        fn replace_append(&mut self, _caps: &Captures<'_>, _dst: &mut String) {
            // implementation not needed
        }

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            None
        }
    }

    let mut replacer = SimpleReplacer;
    let replacer_ref = replacer.by_ref();
}

#[test]
fn test_replacer_by_ref_with_empty_state() {
    struct EmptyReplacer;

    impl Replacer for EmptyReplacer {
        fn replace_append(&mut self, _caps: &Captures<'_>, _dst: &mut String) {
            // implementation not needed
        }

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            Some(Cow::Borrowed(""))
        }
    }

    let mut empty_replacer = EmptyReplacer;
    let replacer_ref = empty_replacer.by_ref();
}

#[test]
fn test_replacer_by_ref_with_no_expansion() {
    struct NoExpansionReplacer;

    impl Replacer for NoExpansionReplacer {
        fn replace_append(&mut self, _caps: &Captures<'_>, _dst: &mut String) {
            // implementation not needed
        }

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            Some(Cow::Owned(String::from("No expansion")))
        }
    }

    let mut no_expansion_replacer = NoExpansionReplacer;
    let replacer_ref = no_expansion_replacer.by_ref();
}

#[test]
fn test_replacer_by_ref_with_complex_state() {
    struct ComplexReplacer {
        state: String,
    }

    impl Replacer for ComplexReplacer {
        fn replace_append(&mut self, _caps: &Captures<'_>, _dst: &mut String) {
            // implementation not needed
        }

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            Some(Cow::Borrowed(&self.state))
        }
    }

    let mut complex_replacer = ComplexReplacer { state: String::from("Complex") };
    let replacer_ref = complex_replacer.by_ref();
}

