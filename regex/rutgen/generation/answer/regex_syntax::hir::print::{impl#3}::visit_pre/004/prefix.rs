// Answer 0

#[test]
fn test_visit_pre_concat_valid() {
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

    let hir1 = Hir { kind: HirKind::Empty, props: Properties::default() };
    let hir2 = Hir { kind: HirKind::Literal(vec![b'a']), props: Properties::default() };

    let concat_hir = Hir {
        kind: HirKind::Concat(vec![hir1.clone(), hir2.clone()]),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    
    let _ = visitor.visit_pre(&concat_hir);
}

#[test]
fn test_visit_pre_concat_multiple_elements() {
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

    let hir1 = Hir { kind: HirKind::Class(Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]))) , props: Properties::default() };
    let hir2 = Hir { kind: HirKind::Class(Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(0x61, 0x7A)]))), props: Properties::default() };

    let concat_hir = Hir {
        kind: HirKind::Concat(vec![hir1.clone(), hir2.clone()]),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    
    let _ = visitor.visit_pre(&concat_hir);
}

