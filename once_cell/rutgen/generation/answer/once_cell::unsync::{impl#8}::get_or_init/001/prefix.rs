// Answer 0

#[test]
fn test_get_or_init_with_panic() {
    struct TestType;
    let cell: OnceCell<TestType> = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("This is a panic for testing"));
}

#[test]
#[should_panic]
fn test_get_or_init_reentrant_initialization() {
    struct TestType;
    let cell: OnceCell<TestType> = OnceCell::new();
    let _value = cell.get_or_init(|| {
        cell.get_or_init(|| panic!("This should panic due to reentrancy"));
        TestType
    });
}

