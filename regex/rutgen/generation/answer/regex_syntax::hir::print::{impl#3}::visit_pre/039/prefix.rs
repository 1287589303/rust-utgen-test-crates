// Answer 0

#[test]
fn test_visit_pre_look_endlf() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let hir = Hir {
        kind: HirKind::Look(hir::Look::EndLF),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_empty_writer() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let hir = Hir {
        kind: HirKind::Look(hir::Look::EndLF),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
    let _ = visitor.finish(); 
}

