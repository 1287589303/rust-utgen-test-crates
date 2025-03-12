// Answer 0

#[test]
fn test_c_look_start() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("test"));
    let look = Look::Start;
    let hir = Hir::look(look);
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_look_end() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("test"));
    let look = Look::End;
    let hir = Hir::look(look);
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_look_start_end() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("test"));
    let look = Look::Start | Look::End;
    let hir = Hir::look(look);
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_look_word() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("test"));
    let look = Look::Word;
    let hir = Hir::look(look);
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_look_word_negate() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("test"));
    let look = Look::WordNegate;
    let hir = Hir::look(look);
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_look_combined_flags() {
    let config = Config { size_limit: None };
    let compiler = Compiler::new(config, String::from("test"));
    let look = Look::Start | Look::Word | Look::End;
    let hir = Hir::look(look);
    let _ = compiler.c(&hir);
}

