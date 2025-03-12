// Answer 0

#[test]
fn test_fail_function() {
    let result = Hir::fail();
}

#[test]
fn test_fail_function_properties() {
    let result = Hir::fail();
    let class = Class::Bytes(ClassBytes::empty());
    let props = Properties::class(&class);
}

