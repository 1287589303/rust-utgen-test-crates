// Answer 0

#[test]
fn test_visit_pre_look_word_end_half_unicode_fail() {
    struct TestWriter;
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let writer = TestWriter;
    let mut visitor = Writer { wtr: writer };
    
    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordEndHalfUnicode),
        props: Default::default(),
    };

    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_word_end_half_unicode_success() {
    struct TestWriter;
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let writer = TestWriter;
    let mut visitor = Writer { wtr: writer };
    
    let hir = Hir {
        kind: HirKind::Look(hir::Look::WordEndHalfUnicode),
        props: Default::default(),
    };

    let _ = visitor.visit_pre(&hir);
}

