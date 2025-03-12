// Answer 0

#[test]
fn test_visit_pre_class_unicode() {
    struct InnerWriter {
        output: String,
    }

    impl fmt::Write for InnerWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = InnerWriter { output: String::new() };

    let range1 = ClassUnicodeRange::new('a', 'c');
    let range2 = ClassUnicodeRange::new('e', 'g');
    let class_unicode = ClassUnicode::new(vec![range1, range2]);

    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(class_unicode)),
        props: Default::default(),
    };

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    struct InnerWriter {
        output: String,
    }

    impl fmt::Write for InnerWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = InnerWriter { output: String::new() };

    let range1 = ClassBytesRange::new(1, 3);
    let range2 = ClassBytesRange::new(5, 7);
    let class_bytes = ClassBytes::new(vec![range1, range2]);

    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(class_bytes)),
        props: Default::default(),
    };

    writer.visit_pre(&hir).unwrap();
}

