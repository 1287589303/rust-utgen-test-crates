// Answer 0

#[test]
fn test_c_class_empty_ranges() {
    let config = Config { nest_limit: 10 };
    let compiler = Compiler::new(config, String::from(""));

    let empty_class = hir::Class { ranges: Vec::new() };

    let result = compiler.c_class(&empty_class);
}

#[test]
fn test_c_class_fail_state() {
    let config = Config { nest_limit: 10 };
    let compiler = Compiler::new(config, String::from(""));

    let empty_class = hir::Class { ranges: Vec::new() };

    let result = compiler.c_class(&empty_class);
}

