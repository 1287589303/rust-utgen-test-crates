// Answer 0

#[test]
fn test_visit_pre_look_start_error() {
    struct TestWriter {
        output: Result<(), fmt::Error>,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            self.output = Err(fmt::Error);
            self.output
        }
    }

    let mut writer = TestWriter { output: Ok(()) };
    let hir = Hir {
        kind: HirKind::Look(hir::Look::Start),
        props: Properties::default(),
    };

    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_end_error() {
    struct TestWriter {
        output: Result<(), fmt::Error>,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            self.output = Err(fmt::Error);
            self.output
        }
    }

    let mut writer = TestWriter { output: Ok(()) };
    let hir = Hir {
        kind: HirKind::Look(hir::Look::End),
        props: Properties::default(),
    };

    let _ = writer.visit_pre(&hir);
} 

#[test]
fn test_visit_pre_look_ascii_word_error() {
    struct TestWriter {
        output: Result<(), fmt::Error>,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            self.output = Err(fmt::Error);
            self.output
        }
    }

    let mut writer = TestWriter { output: Ok(()) };
    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordAscii),
        props: Properties::default(),
    };

    let _ = writer.visit_pre(&hir);
} 

#[test]
fn test_visit_pre_look_unicode_word_error() {
    struct TestWriter {
        output: Result<(), fmt::Error>,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            self.output = Err(fmt::Error);
            self.output
        }
    }

    let mut writer = TestWriter { output: Ok(()) };
    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordUnicode),
        props: Properties::default(),
    };

    let _ = writer.visit_pre(&hir);
}

