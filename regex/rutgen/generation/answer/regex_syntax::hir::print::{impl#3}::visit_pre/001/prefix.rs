// Answer 0

#[test]
fn test_visit_pre_alternation_non_empty() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let hir1 = Hir {
        kind: HirKind::Literal(hir::Literal(b"a".to_vec())),
        props: Properties::default(),
    };

    let hir2 = Hir {
        kind: HirKind::Literal(hir::Literal(b"b".to_vec())),
        props: Properties::default(),
    };

    let hir = Hir {
        kind: HirKind::Alternation(vec![hir1, hir2]),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_alternation_with_nested_concats() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let nested_hir1 = Hir {
        kind: HirKind::Concat(vec![
            Hir { kind: HirKind::Literal(hir::Literal(b"x".to_vec())), props: Properties::default() },
            Hir { kind: HirKind::Literal(hir::Literal(b"y".to_vec())), props: Properties::default() },
        ]),
        props: Properties::default(),
    };

    let nested_hir2 = Hir {
        kind: HirKind::Concat(vec![
            Hir { kind: HirKind::Literal(hir::Literal(b"z".to_vec())), props: Properties::default() },
            Hir { kind: HirKind::Literal(hir::Literal(b"w".to_vec())), props: Properties::default() },
        ]),
        props: Properties::default(),
    };

    let hir = Hir {
        kind: HirKind::Alternation(vec![nested_hir1, nested_hir2]),
        props: Properties::default(),
    };
    
    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
}

#[test]
#[should_panic]
fn test_visit_pre_alternation_with_conflicting_chars() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let hir1 = Hir {
        kind: HirKind::Literal(hir::Literal(b"c".to_vec())),
        props: Properties::default(),
    };

    let hir2 = Hir {
        kind: HirKind::Literal(hir::Literal(b"c".to_vec())),
        props: Properties::default(),
    };

    let hir = Hir {
        kind: HirKind::Alternation(vec![hir1, hir2]),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
}

