// Answer 0

#[test]
fn test_visit_pre_word_ascii() {
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
    let look = hir::Look::WordAscii;
    let hir_kind = HirKind::Look(look);
    let properties = Properties::default();
    let hir = Hir { kind: hir_kind, props: properties };

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
    let expected_output = "(?-u:\\b)";
    assert_eq!(writer.output, expected_output);
}

