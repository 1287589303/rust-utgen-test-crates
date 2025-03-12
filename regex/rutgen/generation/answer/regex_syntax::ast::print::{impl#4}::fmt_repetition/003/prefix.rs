// Answer 0

#[test]
fn test_fmt_repetition_range_bounded_err() {
    struct MockWriter {
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail && s == "?" {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { should_fail: true };
    let ast = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(1, 5)),
        },
        greedy: false,
        ast: Box::new(Default::default()),
    };

    let mut fmt_writer = Writer { wtr: writer };
    let _ = fmt_writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_bounded_err_with_different_range() {
    struct MockWriter {
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail && s == "?" {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { should_fail: true };
    let ast = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(0, 10)),
        },
        greedy: false,
        ast: Box::new(Default::default()),
    };

    let mut fmt_writer = Writer { wtr: writer };
    let _ = fmt_writer.fmt_repetition(&ast);
}

