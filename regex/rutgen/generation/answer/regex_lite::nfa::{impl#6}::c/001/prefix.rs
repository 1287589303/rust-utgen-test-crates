// Answer 0

#[test]
fn test_c_alternation_single_empty() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let empty_hir = Hir::empty();
    let alternation_hir = Hir::alternation(vec![empty_hir]);
    let compiler = Compiler::new(config, String::from("pattern"));
    let _ = compiler.c(&alternation_hir);
}

#[test]
fn test_c_alternation_single_char() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let char_hir = Hir::char('a');
    let alternation_hir = Hir::alternation(vec![char_hir]);
    let compiler = Compiler::new(config, String::from("pattern"));
    let _ = compiler.c(&alternation_hir);
}

#[test]
fn test_c_alternation_multiple() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let char_hir_a = Hir::char('a');
    let char_hir_b = Hir::char('b');
    let class_hir = Hir::class(Class { ranges: vec![] });
    let alternation_hir = Hir::alternation(vec![char_hir_a, char_hir_b, class_hir]);
    let compiler = Compiler::new(config, String::from("pattern"));
    let _ = compiler.c(&alternation_hir);
}

#[test]
fn test_c_alternation_multiple_repetitions() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let repetition_hir = Hir::repetition(Repetition { min: 1, max: Some(3), greedy: true, sub: Box::new(Hir::char('c')) });
    let alternation_hir = Hir::alternation(vec![Hir::char('a'), repetition_hir]);
    let compiler = Compiler::new(config, String::from("pattern"));
    let _ = compiler.c(&alternation_hir);
}

#[test]
fn test_c_alternation_with_capture() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let capture_hir = Hir::capture(Capture { index: 0, name: None, sub: Box::new(Hir::char('d')) });
    let alternation_hir = Hir::alternation(vec![capture_hir]);
    let compiler = Compiler::new(config, String::from("pattern"));
    let _ = compiler.c(&alternation_hir);
}

#[test]
fn test_c_alternation_multiple_types() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let empty_hir = Hir::empty();
    let char_hir = Hir::char('a');
    let class_hir = Hir::class(Class { ranges: vec![] });
    let look_hir = Hir::look(Look::Start);
    let repetition_hir = Hir::repetition(Repetition { min: 1, max: None, greedy: false, sub: Box::new(Hir::char('b')) });
    let alternation_hir = Hir::alternation(vec![empty_hir, char_hir, class_hir, look_hir, repetition_hir]);
    let compiler = Compiler::new(config, String::from("pattern"));
    let _ = compiler.c(&alternation_hir);
}

