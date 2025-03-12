// Answer 0

#[test]
fn test_hir_drop_concat_empty() {
    let empty_vec: Vec<Hir> = Vec::new();
    let hir_concat = Hir {
        kind: HirKind::Concat(empty_vec),
        props: Properties(Box::new(PropertiesI::default())),
    };
    let _ = hir_concat; // Call drop by going out of scope
}

