// Answer 0

#[test]
fn test_c_class_single_range() {
    let class_range = ClassRange { start: 'a', end: 'b' };
    let class = Class { ranges: vec![class_range] };
    let config = Config { nest_limit: 10 };
    let compiler = Compiler::new(config, String::from("a-b"));
    let _result = compiler.c_class(&class);
}

#[test]
fn test_c_class_multiple_ranges() {
    let class_ranges = vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'e', end: 'f' },
    ];
    let class = Class { ranges: class_ranges };
    let config = Config { nest_limit: 10 };
    let compiler = Compiler::new(config, String::from("a-c,e-f"));
    let _result = compiler.c_class(&class);
}

#[test]
fn test_c_class_identical_ranges() {
    let class_range = ClassRange { start: 'x', end: 'x' };
    let class = Class { ranges: vec![class_range] };
    let config = Config { nest_limit: 10 };
    let compiler = Compiler::new(config, String::from("x"));
    let _result = compiler.c_class(&class);
}

#[test]
fn test_c_class_reverse_ranges() {
    let class_ranges = vec![
        ClassRange { start: 'd', end: 'f' },
        ClassRange { start: 'a', end: 'c' },
    ];
    let class = Class { ranges: class_ranges };
    let config = Config { nest_limit: 10 };
    let compiler = Compiler::new(config, String::from("d-f,a-c"));
    let _result = compiler.c_class(&class);
}

