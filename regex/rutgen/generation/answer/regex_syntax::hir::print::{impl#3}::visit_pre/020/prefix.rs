// Answer 0

#[test]
fn test_visit_pre_look_word_start_unicode_err() {
    struct TestWriter {
        error: bool,
    }
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            if self.error {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }
    let mut writer = TestWriter { error: true };
    let look = hir::Look::WordStartUnicode;
    let hir = Hir { kind: HirKind::Look(look), props: Properties::default() };
    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_word_start_unicode_success() {
    struct TestWriter {
        error: bool,
    }
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            if self.error {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }
    let mut writer = TestWriter { error: false };
    let look = hir::Look::WordStartUnicode;
    let hir = Hir { kind: HirKind::Look(look), props: Properties::default() };
    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_word_end_unicode_err() {
    struct TestWriter {
        error: bool,
    }
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            if self.error {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }
    let mut writer = TestWriter { error: true };
    let look = hir::Look::WordEndUnicode;
    let hir = Hir { kind: HirKind::Look(look), props: Properties::default() };
    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_word_end_unicode_success() {
    struct TestWriter {
        error: bool,
    }
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            if self.error {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }
    let mut writer = TestWriter { error: false };
    let look = hir::Look::WordEndUnicode;
    let hir = Hir { kind: HirKind::Look(look), props: Properties::default() };
    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

