// Answer 0

#[test]
fn test_visit_post_concat_with_two_exprs() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let expr1 = Hir { kind: HirKind::Empty, props: Properties::default() };
    let expr2 = Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties::default() };
    let concat_hir = Hir {
        kind: HirKind::Concat(vec![expr1, expr2]),
        props: Properties::default(),
    };

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { wtr: writer };

    visitor.visit_post(&concat_hir).unwrap();
}

#[test]
fn test_visit_post_concat_with_multiple_exprs() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let expr1 = Hir { kind: HirKind::Class(Class::new()), props: Properties::default() };
    let expr2 = Hir { kind: HirKind::Look(Look::new()), props: Properties::default() };
    let expr3 = Hir { kind: HirKind::Literal(Literal::new("b")), props: Properties::default() };
    let concat_hir = Hir {
        kind: HirKind::Concat(vec![expr1, expr2, expr3]),
        props: Properties::default(),
    };

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { wtr: writer };

    visitor.visit_post(&concat_hir).unwrap();
}

