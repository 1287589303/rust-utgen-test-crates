// Answer 0

#[test]
fn test_visit_pre_with_capture_name_valid() {
    struct MockWriter {
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { should_fail: true };
    let hir = Hir {
        kind: HirKind::Capture { name: Some(String::from("capture_name")) },
        props: Properties::default()
    };
    writer.visit_pre(&hir).unwrap_err();
}

#[test]
fn test_visit_pre_with_capture_name_none() {
    struct MockWriter {
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { should_fail: true };
    let hir = Hir {
        kind: HirKind::Capture { name: None },
        props: Properties::default()
    };
    writer.visit_pre(&hir).unwrap_err();
}

#[test]
fn test_visit_pre_with_capture_name_no_failure() {
    struct MockWriter {
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { should_fail: false };
    let hir = Hir {
        kind: HirKind::Capture { name: Some(String::from("capture_name")) },
        props: Properties::default()
    };
    writer.visit_pre(&hir).unwrap();
}

