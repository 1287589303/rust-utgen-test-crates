// Answer 0

#[test]
fn test_visit_pre_look_start_crlf_err() {
    struct TestWriter {
        output: String,
        write_result: fmt::Result,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.write_result = Err(fmt::Error);
            self.output.push_str(s);
            self.write_result
        }
    }

    let writer = TestWriter { output: String::new(), write_result: Ok(()) };
    let mut writer = Writer { wtr: writer };

    let hir = Hir {
        kind: HirKind::Look(hir::Look::StartCRLF),
        props: Default::default(),
    };

    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_end_crlf_err() {
    struct TestWriter {
        output: String,
        write_result: fmt::Result,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.write_result = Err(fmt::Error);
            self.output.push_str(s);
            self.write_result
        }
    }

    let writer = TestWriter { output: String::new(), write_result: Ok(()) };
    let mut writer = Writer { wtr: writer };

    let hir = Hir {
        kind: HirKind::Look(hir::Look::EndCRLF),
        props: Default::default(),
    };

    let _ = writer.visit_pre(&hir);
}

