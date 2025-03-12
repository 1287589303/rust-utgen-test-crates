// Answer 0

#[test]
fn test_c_with_repetition() {
    let config = Config { size_limit: None };
    let pattern = String::from("a*");
    let compiler = Compiler::new(config, pattern);

    let sub_capture = Hir::capture(Capture {
        index: 0,
        name: Some(Box::from("capture")),
        sub: Box::new(Hir::char('a')),
    });

    let repetition = Hir::repetition(Repetition {
        min: 0,
        max: Some(10),
        greedy: true,
        sub: Box::new(sub_capture),
    });

    let result = compiler.c(&repetition);
}

#[test]
fn test_c_with_zero_or_more_repetition() {
    let config = Config { size_limit: None };
    let pattern = String::from("a*?");
    let compiler = Compiler::new(config, pattern);

    let sub_capture = Hir::capture(Capture {
        index: 1,
        name: Some(Box::from("capture1")),
        sub: Box::new(Hir::char('a')),
    });

    let repetition = Hir::repetition(Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(sub_capture),
    });

    let result = compiler.c(&repetition);
}

