// Answer 0

#[test]
fn test_visit_pre_look_endlf_success() {
    struct MockWriter {
        buffer: String,
        capacity: usize,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.buffer.len() + s.len() > self.capacity {
                return Err(fmt::Error);
            }
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter {
        buffer: String::new(),
        capacity: 100,
    };

    let hir = hir::Hir {
        kind: hir::HirKind::Look(hir::Look::EndLF),
        props: Default::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_endlf_error() {
    struct MockWriter {
        buffer: String,
        capacity: usize,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut writer = MockWriter {
        buffer: String::new(),
        capacity: 0,
    };

    let hir = hir::Hir {
        kind: hir::HirKind::Look(hir::Look::EndLF),
        props: Default::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

