// Answer 0

#[test]
fn test_visit_pre_look_end_fail() {
    struct MockWriter {
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { should_fail: true };
    let look = hir::Look::End;
    let hir = Hir {
        kind: HirKind::Look(look),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_look_end_success() {
    struct MockWriter {
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { should_fail: false };
    let look = hir::Look::End;
    let hir = Hir {
        kind: HirKind::Look(look),
        props: Properties::default(),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

