// Answer 0

#[test]
fn test_visit_pre_empty() {
    struct TestVisitor {
        output: String,
    }

    impl fmt::Write for TestVisitor {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut visitor = TestVisitor { output: String::new() };
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };

    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class() {
    struct TestVisitor {
        output: String,
    }
    
    impl fmt::Write for TestVisitor {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange::new('a', 'a'),
        ClassUnicodeRange::new('b', 'c'),
    ]);

    let mut visitor = TestVisitor { output: String::new() };
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(class_unicode)),
        props: Properties::default(),
    };

    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class() {
    struct TestVisitor {
        output: String,
    }
    
    impl fmt::Write for TestVisitor {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut class_bytes = ClassBytes::new(vec![
        ClassBytesRange::new(1, 3),
        ClassBytesRange::new(4, 5),
    ]);

    let mut visitor = TestVisitor { output: String::new() };
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(class_bytes)),
        props: Properties::default(),
    };

    visitor.visit_pre(&hir).unwrap();
}

