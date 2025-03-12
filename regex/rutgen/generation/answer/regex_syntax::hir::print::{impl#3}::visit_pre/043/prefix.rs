// Answer 0

#[test]
fn test_visit_pre_look_end() {
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let hir = hir::Hir {
        kind: hir::HirKind::Look(hir::Look::End),
        props: Default::default(),
    };

    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
    
    // At this point, you can check the writer.output to see if it matched r"\z"
}

