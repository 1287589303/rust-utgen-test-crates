// Answer 0

#[test]
fn test_c_concat_with_empty_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = ".*";
    let hir = Hir::concat(vec![Hir::empty(), Hir::char('a')]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_concat_with_char_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = "a";
    let hir = Hir::concat(vec![Hir::char('a'), Hir::char('b')]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_concat_with_class_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = "[a-z]";
    let class_hir = Hir::class(Class { ranges: vec![] });
    let hir = Hir::concat(vec![class_hir, Hir::char('c')]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_concat_with_look_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = "^a";
    let look_hir = Hir::look(Look::Start);
    let hir = Hir::concat(vec![look_hir, Hir::char('b')]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_concat_with_repetition_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = "a*";
    let rep_hir = Hir::repetition(Repetition { min: 0, max: None, greedy: true, sub: Box::new(Hir::char('a')) });
    let hir = Hir::concat(vec![rep_hir, Hir::char('b')]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_concat_with_capture_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = "(abc)";
    let capture_hir = Hir::capture(Capture { index: 0, name: None, sub: Box::new(Hir::concat(vec![Hir::char('a'), Hir::char('b'), Hir::char('c')])) });
    let hir = Hir::concat(vec![capture_hir, Hir::char('d')]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_concat_with_nested_concat_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = "ab|cd";
    let nested_hir = Hir::concat(vec![Hir::char('c'), Hir::char('d')]);
    let hir = Hir::concat(vec![Hir::char('a'), Hir::char('b'), nested_hir]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_concat_with_alternation_hir() {
    let config = Config { size_limit: None, nest_limit: 5, flags: Flags::default() };
    let pattern = "a|b";
    let alt_hir = Hir::alternation(vec![Hir::char('a'), Hir::char('b')]);
    let hir = Hir::concat(vec![Hir::char('x'), alt_hir]);
    let compiler = Compiler::new(config, pattern.to_string());
    let _ = compiler.c(&hir);
}

