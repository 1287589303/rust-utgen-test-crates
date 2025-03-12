// Answer 0

#[test]
fn test_drop_empty() {
    let hir = Hir::empty();
    drop(&mut hir.clone());
}

#[test]
fn test_drop_char() {
    let hir = Hir::char('a');
    drop(&mut hir.clone());
}

#[test]
fn test_drop_class() {
    struct MockClass;
    let class_instance = MockClass;
    let hir = Hir::class(class_instance);
    drop(&mut hir.clone());
}

#[test]
fn test_drop_look() {
    struct MockLook;
    let look_instance = MockLook;
    let hir = Hir::look(look_instance);
    drop(&mut hir.clone());
}

