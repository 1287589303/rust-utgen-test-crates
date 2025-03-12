// Answer 0

#[test]
fn test_visit_pre_literal() {
    let trans = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags::default()), 
        utf8: true, 
        line_terminator: b'\n' 
    };
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    let ast = Ast::Literal(Box::new(ast::Literal::new("test".to_string())));
    let _ = translator_i.visit_pre(&ast);
}

#[test]
fn test_visit_pre_flags() {
    let trans = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags::default()), 
        utf8: true, 
        line_terminator: b'\n' 
    };
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    let ast = Ast::Flags(Box::new(ast::SetFlags::new(vec![])));
    let _ = translator_i.visit_pre(&ast);
}

