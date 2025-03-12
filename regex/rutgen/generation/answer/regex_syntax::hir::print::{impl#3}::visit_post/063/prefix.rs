// Answer 0

#[test]
fn test_visit_post_repetition_0_to_1_greedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            min: 0,
            max: Some(1),
            greedy: true,
            sub: Box::new(Hir {
                kind: HirKind::Empty,
                props: Properties::default(),
            }),
        }),
        props: Properties::default(),
    };

    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_0_none_greedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            min: 0,
            max: None,
            greedy: true,
            sub: Box::new(Hir {
                kind: HirKind::Empty,
                props: Properties::default(),
            }),
        }),
        props: Properties::default(),
    };

    let _ = writer.visit_post(&hir);
}

