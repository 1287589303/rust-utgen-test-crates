// Answer 0

#[test]
fn test_visit_post_repetition_0_none() {
    struct TestWriter {
        output: String,
        error: Option<fmt::Error>,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if let Some(_) = self.error {
                return Err(fmt::Error);
            }
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new(), error: None };

    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Empty,
            props: Properties::default(),
        }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };

    let _ = writer.write_str("*");
    let mut visitor = Writer { wtr: writer };

    let _ = visitor.visit_post(&hir);
}

