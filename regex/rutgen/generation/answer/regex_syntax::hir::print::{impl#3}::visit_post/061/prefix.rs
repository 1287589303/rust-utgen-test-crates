// Answer 0

#[test]
fn test_visit_post_repetition_min_zero_max_none_greedy_false() {
    struct TestWriter {
        buffer: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: String::new() };
    
    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: writer };
    let _result = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_min_zero_max_none_greedy_false_large() {
    struct TestWriter {
        buffer: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: String::new() };

    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: writer };
    let _result = visitor.visit_post(&hir);
}

