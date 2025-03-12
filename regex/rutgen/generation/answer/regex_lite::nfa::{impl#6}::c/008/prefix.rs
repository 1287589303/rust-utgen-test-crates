// Answer 0

#[test]
fn test_c_empty() {
    let config = Config { size_limit: None };
    let pattern = String::from("");
    let compiler = Compiler::new(config, pattern);
    let hir = Hir::empty();
    let _result = compiler.c(&hir);
}

#[test]
fn test_c_char() {
    let config = Config { size_limit: None };
    let pattern = String::from("a");
    let compiler = Compiler::new(config, pattern);
    let hir = Hir::char('a');
    let _result = compiler.c(&hir);
}

#[test]
fn test_c_class() {
    let config = Config { size_limit: None };
    let pattern = String::from("[a-z]");
    let compiler = Compiler::new(config, pattern);
    let class = Class { ranges: vec![] };
    let hir = Hir::class(class);
    let _result = compiler.c(&hir);
}

#[test]
fn test_c_look() {
    let config = Config { size_limit: None };
    let pattern = String::from("^");
    let compiler = Compiler::new(config, pattern);
    let hir = Hir::look(Look::Start);
    let _result = compiler.c(&hir);
}

#[test]
fn test_c_repetition() {
    let config = Config { size_limit: None };
    let pattern = String::from("a*");
    let compiler = Compiler::new(config, pattern);
    let repetition = Repetition { min: 0, max: None, greedy: true, sub: Box::new(Hir::char('a')) };
    let hir = Hir::repetition(repetition);
    let _result = compiler.c(&hir);
}

#[test]
fn test_c_capture() {
    let config = Config { size_limit: None };
    let pattern = String::from("(a)");
    let compiler = Compiler::new(config, pattern);
    let capture = Capture { index: 0, name: None, sub: Box::new(Hir::char('a')) };
    let hir = Hir::capture(capture);
    let _result = compiler.c(&hir);
}

#[test]
fn test_c_concat() {
    let config = Config { size_limit: None };
    let pattern = String::from("ab");
    let compiler = Compiler::new(config, pattern);
    let hir = Hir::concat(vec![Hir::char('a'), Hir::char('b')]);
    let _result = compiler.c(&hir);
}

#[test]
fn test_c_alternation() {
    let config = Config { size_limit: None };
    let pattern = String::from("a|b");
    let compiler = Compiler::new(config, pattern);
    let hir = Hir::alternation(vec![Hir::char('a'), Hir::char('b')]);
    let _result = compiler.c(&hir);
}

