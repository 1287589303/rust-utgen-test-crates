// Answer 0

#[test]
fn test_visit_pre_look_word_end_ascii() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let look = hir::Look::WordEndAscii;
    let hir = Hir {
        kind: HirKind::Look(look),
        props: Properties::default(),
    };

    let visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_word_end_unicode() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let look = hir::Look::WordEndUnicode;
    let hir = Hir {
        kind: HirKind::Look(look),
        props: Properties::default(),
    };

    let visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_word_start_half_ascii() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let look = hir::Look::WordStartHalfAscii;
    let hir = Hir {
        kind: HirKind::Look(look),
        props: Properties::default(),
    };

    let visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_word_end_half_unicode() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let look = hir::Look::WordEndHalfUnicode;
    let hir = Hir {
        kind: HirKind::Look(look),
        props: Properties::default(),
    };

    let visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

