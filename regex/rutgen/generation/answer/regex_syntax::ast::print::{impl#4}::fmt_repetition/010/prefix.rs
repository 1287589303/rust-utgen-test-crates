// Answer 0

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_zero_or_one_not_greedy() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: false,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_zero_or_more_not_greedy() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_one_or_more_not_greedy() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_exactly() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(3)) },
        greedy: false,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_at_least() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(2)) },
        greedy: false,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_bounded() {
    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = fmt::Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(1, 4)) },
        greedy: false,
        ast: Box::new(ast::Literal::new("a")),
    };

    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_repetition(&ast);
}

