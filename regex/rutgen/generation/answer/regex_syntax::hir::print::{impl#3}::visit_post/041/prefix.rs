// Answer 0

#[test]
fn test_visit_post_repetition_min_1_max_none_greedy_false() {
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
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_1_max_none_greedy_true() {
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
        sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_1_max_1_greedy_false() {
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
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_1_max_1_greedy_true() {
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
        sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    visitor.visit_post(&hir).unwrap();
}

