// Answer 0

#[test]
fn test_visit_post_repetition_min_0_max_1() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    
    let repetition = Repetition {
        min: 0,
        max: Some(1),
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

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_min_0_max_none() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    
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

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_min_1_max_none() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    
    let repetition = Repetition {
        min: 1,
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

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_min_1_max_1() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    
    let repetition = Repetition {
        min: 1,
        max: Some(1),
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

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_post(&hir);
}

