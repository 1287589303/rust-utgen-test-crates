// Answer 0

#[test]
fn test_c_char_valid_a() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("a"));
    let hir = Hir::char('a');
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_char_valid_b() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("b"));
    let hir = Hir::char('b');
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_char_valid_unicode() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("中"));
    let hir = Hir::char('中');
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_char_special_character() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("!"));
    let hir = Hir::char('!');
    let _ = compiler.c(&hir);
}

