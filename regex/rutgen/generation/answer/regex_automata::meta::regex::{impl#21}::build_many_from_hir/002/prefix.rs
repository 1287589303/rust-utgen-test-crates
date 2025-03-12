// Answer 0

#[test]
fn test_build_many_from_hir_valid_hirs() {
    use regex_syntax::hir::{Hir, Look};

    let hir1 = Hir::concat(vec![
        Hir::look(Look::StartCRLF),
        Hir::literal("foo".as_bytes()),
        Hir::look(Look::EndCRLF),
    ]);
    let hir2 = Hir::concat(vec![
        Hir::look(Look::StartCRLF),
        Hir::literal("bar".as_bytes()),
        Hir::look(Look::EndCRLF),
    ]);

    let builder = Builder::new()
        .configure(Config {
            match_kind: Some(MatchKind::All),
            ..Default::default()
        });

    let _regex = builder.build_many_from_hir(&[&hir1, &hir2]).unwrap();
}

#[test]
fn test_build_many_from_hir_single_hir() {
    use regex_syntax::hir::{Hir, Look};

    let hir = Hir::concat(vec![
        Hir::look(Look::StartCRLF),
        Hir::literal("baz".as_bytes()),
        Hir::look(Look::EndCRLF),
    ]);

    let builder = Builder::new()
        .configure(Config {
            match_kind: Some(MatchKind::All),
            ..Default::default()
        });

    let _regex = builder.build_many_from_hir(&[&hir]).unwrap();
}

#[test]
fn test_build_many_from_hir_empty_hirs_array() {
    let builder = Builder::new()
        .configure(Config {
            match_kind: Some(MatchKind::All),
            ..Default::default()
        });

    let result = builder.build_many_from_hir::<&Hir>(&[]);
    assert!(result.is_err()); // Expects a BuildError
}

#[test]
fn test_build_many_from_hir_valid_hirs_with_charset() {
    use regex_syntax::hir::{Hir, Look};

    let hir1 = Hir::concat(vec![
        Hir::look(Look::StartCRLF),
        Hir::literal("abc".as_bytes()),
        Hir::look(Look::EndCRLF),
    ]);
    let hir2 = Hir::concat(vec![
        Hir::look(Look::StartCRLF),
        Hir::literal("xyz".as_bytes()),
        Hir::look(Look::EndCRLF),
    ]);
    
    let builder = Builder::new()
        .configure(Config {
            match_kind: Some(MatchKind::All),
            ..Default::default()
        });

    let _regex = builder.build_many_from_hir(&[&hir1, &hir2]).unwrap();
}

