// Answer 0

#[test]
fn test_new_empty_slice() {
    struct TestType;
    
    let result: &Slice<TestType> = Slice::new();
}

#[test]
fn test_new_empty_slice_with_different_type() {
    struct AnotherType;

    let result: &Slice<AnotherType> = Slice::new();
}

