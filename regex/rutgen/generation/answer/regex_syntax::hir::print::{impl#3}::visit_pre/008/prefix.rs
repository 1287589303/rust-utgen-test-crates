// Answer 0

#[test]
fn test_visit_pre_with_capture() {
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
    let capture_hir = Hir {
        kind: HirKind::Capture(hir::Capture { name: Some(String::from("test_capture")), ..Default::default() }),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    visitor.visit_pre(&capture_hir).unwrap();
}

#[test]
fn test_visit_pre_with_another_capture() {
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
    let capture_hir = Hir {
        kind: HirKind::Capture(hir::Capture { name: Some(String::from("another_capture")), ..Default::default() }),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    visitor.visit_pre(&capture_hir).unwrap();
}

