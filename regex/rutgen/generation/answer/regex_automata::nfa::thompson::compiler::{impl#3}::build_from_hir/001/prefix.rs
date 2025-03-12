// Answer 0

#[test]
fn test_build_from_hir_empty_pattern() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![])));
    let compiler = Compiler::new();
    let config = NFA::config().nfa_size_limit(Some(1000));
    let _ = compiler.configure(config).build_from_hir(&hir);
}

#[test]
fn test_build_from_hir_single_character() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(b'a', b'a')])));
    let compiler = Compiler::new();
    let config = NFA::config().nfa_size_limit(Some(1000));
    let _ = compiler.configure(config).build_from_hir(&hir);
}

#[test]
fn test_build_from_hir_multi_character() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'a', b'z'),
        ClassBytesRange::new(b'A', b'Z'),
    ])));
    let compiler = Compiler::new();
    let config = NFA::config().nfa_size_limit(Some(1000));
    let _ = compiler.configure(config).build_from_hir(&hir);
}

#[test]
fn test_build_from_hir_character_classes() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'0', b'9'),
        ClassBytesRange::new(b'A', b'Z'),
        ClassBytesRange::new(b'a', b'z'),
    ])));
    let compiler = Compiler::new();
    let config = NFA::config().nfa_size_limit(Some(1000));
    let _ = compiler.configure(config).build_from_hir(&hir);
}

#[test]
fn test_build_from_hir_large_size_limit() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'0', b'9'),
        ClassBytesRange::new(b'A', b'Z'),
        ClassBytesRange::new(b'a', b'z'),
    ])));
    let compiler = Compiler::new();
    let config = NFA::config().nfa_size_limit(Some(10000));
    let _ = compiler.configure(config).build_from_hir(&hir);
}

