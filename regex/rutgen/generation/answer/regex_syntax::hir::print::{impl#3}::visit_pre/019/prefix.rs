// Answer 0

#[test]
fn test_visit_pre_word_end_unicode() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordEndUnicode),
        props: Default::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_word_end_unicode_multiple_calls() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordEndUnicode),
        props: Default::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_pre_word_end_unicode_with_empty_hir() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordEndUnicode),
        props: Default::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
    let _ = visitor.finish();
}

#[test]
fn test_visit_pre_word_end_unicode_verify_output() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordEndUnicode),
        props: Default::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    
    let _ = visitor.visit_pre(&hir);

    assert_eq!(writer.output, r"\b{end}");
}

