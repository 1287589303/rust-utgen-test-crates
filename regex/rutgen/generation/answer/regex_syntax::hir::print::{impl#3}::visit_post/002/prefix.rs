// Answer 0

#[test]
fn test_visit_post_alternation() {
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
    let mut visitor = Writer { wtr: &mut writer };

    let hir = Hir {
        kind: HirKind::Alternation(vec![]),
        props: Properties::default(),
    };

    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_alternation_with_sub() {
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
    let mut visitor = Writer { wtr: &mut writer };

    let hir = Hir {
        kind: HirKind::Alternation(vec![Hir {
            kind: HirKind::Literal(Literal::new("test")),
            props: Properties::default(),
        }]),
        props: Properties::default(),
    };

    let _ = visitor.visit_post(&hir);
}

