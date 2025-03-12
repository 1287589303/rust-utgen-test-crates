// Answer 0

#[test]
fn test_builder_default() {
    let builder = Builder::default();
}

#[test]
fn test_builder_default_pats() {
    let builder = Builder::default();
    let _ = builder.pats.clone();
}

#[test]
fn test_builder_default_metac_nfa_size_limit() {
    let builder = Builder::default();
    let _ = builder.metac.nfa_size_limit;
}

#[test]
fn test_builder_default_metac_hybrid_cache_capacity() {
    let builder = Builder::default();
    let _ = builder.metac.hybrid_cache_capacity;
}

#[test]
fn test_builder_default_syntaxc() {
    let builder = Builder::default();
    let _ = builder.syntaxc.clone();
}

