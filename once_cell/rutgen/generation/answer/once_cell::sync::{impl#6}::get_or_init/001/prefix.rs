// Answer 0

#[test]
#[should_panic]
fn test_get_or_init_with_initialized_cell_panics() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::with_value(TestStruct { value: 42 });
    let _value = cell.get_or_init(|| {
        panic!("This function will panic");
    });
}

#[test]
fn test_get_or_init_with_initialized_cell_returns_existing_value() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::with_value(TestStruct { value: 42 });
    let value = cell.get_or_init(|| TestStruct { value: 0 });
    let _ = value; // Just to use the value
}

#[test]
#[should_panic]
fn test_get_or_init_with_initialized_cell_reentrant_initialization_panics() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::with_value(TestStruct { value: 42 });
    let _value = cell.get_or_init(|| {
        cell.get_or_init(|| TestStruct { value: 7 });
        TestStruct { value: 0 }
    });
}

