// Answer 0

#[test]
fn test_new_from_nfa_valid_small() {
    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'0', b'9'),
    ])));
    let config = NFA::config().nfa_size_limit(Some(1));
    let nfa = NFA::compiler().configure(config).build_from_hir(&hir).unwrap();
    let re = BoundedBacktracker::new_from_nfa(nfa).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.try_captures(&mut cache, "0", &mut caps).unwrap();
}

#[test]
fn test_new_from_nfa_valid_large() {
    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'A', b'Z'),
        ClassBytesRange::new(b'a', b'z'),
        ClassBytesRange::new(b'_', b'_'),
    ])));
    let config = NFA::config().nfa_size_limit(Some(1_000));
    let nfa = NFA::compiler().configure(config).build_from_hir(&hir).unwrap();
    let re = BoundedBacktracker::new_from_nfa(nfa).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.try_captures(&mut cache, "A_b", &mut caps).unwrap();
}

#[test]
fn test_new_from_nfa_invalid() {
    let config = NFA::config().nfa_size_limit(Some(0)); // Size limit set to 0 (invalid)
    let result = NFA::compiler().configure(config).build_from_hir(&Hir::class(Class::Bytes(ClassBytes::new(vec![]))));
    match result {
        Err(BuildError { .. }) => {},
        _ => panic!("Expected a BuildError"),
    }
}

#[test]
fn test_new_from_nfa_patterns() {
    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'a', b'z'),
        ClassBytesRange::new(b'0', b'9'),
    ])));
    let config = NFA::config().nfa_size_limit(Some(100));
    let nfa = NFA::compiler().configure(config).build_from_hir(&hir).unwrap();
    let re = BoundedBacktracker::new_from_nfa(nfa).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.try_captures(&mut cache, "a1", &mut caps).unwrap();
}

#[test]
fn test_new_from_nfa_edge_byte_class() {
    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'_', b'_'),
        ClassBytesRange::new(b'0', b'9'),
        ClassBytesRange::new(b'A', b'Z'),
        ClassBytesRange::new(b'a', b'z'),
    ])));
    let config = NFA::config().nfa_size_limit(Some(100));
    let nfa = NFA::compiler().configure(config).build_from_hir(&hir).unwrap();
    let re = BoundedBacktracker::new_from_nfa(nfa).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.try_captures(&mut cache, "_A1", &mut caps).unwrap();
}

