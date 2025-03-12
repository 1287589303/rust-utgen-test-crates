// Answer 0

#[test]
fn test_visit_post_repetition_zero_min_some_max_equal() {
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
            sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
        }),
        props: Properties::default(),
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_zero_min_some_max_not_equal() {
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
            max: Some(2),
            greedy: true,
            sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
        }),
        props: Properties::default(),
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_zero_min_some_max_greedy_false() {
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
            max: Some(3),
            greedy: false,
            sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }),
        }),
        props: Properties::default(),
    };
    writer.visit_post(&hir).unwrap();
}

