// Answer 0

#[test]
fn test_visit_post_repetition_min_1_max_1() {
    struct TestWriter(Vec<u8>);

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.extend_from_slice(s.as_bytes());
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    
    let repetition = Repetition {
        min: 1,
        max: Some(1),
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Literal(Literal::new("a")),
            props: Properties::new(),
        }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::new(),
    };

    let mut visitor = Writer { wtr: writer };
    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_non_greedy_min_1_max_1() {
    struct TestWriter(Vec<u8>);

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.extend_from_slice(s.as_bytes());
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    
    let repetition = Repetition {
        min: 1,
        max: Some(1),
        greedy: false,
        sub: Box::new(Hir {
            kind: HirKind::Literal(Literal::new("a")),
            props: Properties::new(),
        }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::new(),
    };

    let mut visitor = Writer { wtr: writer };
    visitor.visit_post(&hir).unwrap();
}

