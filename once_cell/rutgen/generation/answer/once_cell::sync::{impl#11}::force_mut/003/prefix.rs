// Answer 0

#[test]
fn test_force_mut_with_initialized_value() {
    struct MyValue {
        value: i32,
    }

    let mut lazy = Lazy {
        cell: OnceCell::with_value(MyValue { value: 42 }),
        init: Cell::new(None),
    };

    let result = Lazy::force_mut(&mut lazy);
    // The result would be &mut MyValue, but we focus only on the call.
}

#[test]
fn test_force_mut_with_non_empty_cell() {
    struct MyValue {
        value: String,
    }

    let mut lazy = Lazy {
        cell: OnceCell::with_value(MyValue { value: "Hello".to_string() }),
        init: Cell::new(None),
    };

    let result = Lazy::force_mut(&mut lazy);
    // The result would be &mut MyValue, but we focus only on the call.
}

