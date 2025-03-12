// Answer 0

#[test]
fn test_build_many_from_hir_valid_input() {
    let compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![
        Hir::class(Class::Bytes(ClassBytes::new(vec![
            ClassBytesRange::new(b'\t', b'\r'),
            ClassBytesRange::new(b' ', b' ')
        ]))),
        Hir::class(Class::Bytes(ClassBytes::new(vec![
            ClassBytesRange::new(b'0', b'9'),
            ClassBytesRange::new(b'A', b'Z'),
            ClassBytesRange::new(b'_', b'_'),
            ClassBytesRange::new(b'a', b'z')
        ]))),
    ];
    let config = Config {
        nfa_size_limit: Some(1_000),
        ..Config::default()
    };
    let nfa = compiler.configure(config).build_many_from_hir(&exprs).unwrap();
}

#[test]
#[should_panic]
fn test_build_many_from_hir_empty_input() {
    let compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![];
    let config = Config {
        nfa_size_limit: Some(1_000),
        ..Config::default()
    };
    compiler.configure(config).build_many_from_hir(&exprs).unwrap();
}

#[test]
#[should_panic]
fn test_build_many_from_hir_invalid_nfa_size_limit() {
    let compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![
        Hir::class(Class::Bytes(ClassBytes::new(vec![
            ClassBytesRange::new(b'\t', b'\r'),
            ClassBytesRange::new(b' ', b' ')
        ]))),
    ];
    let config = Config {
        nfa_size_limit: Some(0),
        ..Config::default()
    };
    compiler.configure(config).build_many_from_hir(&exprs).unwrap();
}

