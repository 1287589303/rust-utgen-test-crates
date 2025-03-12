// Answer 0

#[test]
fn test_force_with_integer_function() {
    struct TestFunction;

    let lazy = Lazy {
        cell: OnceCell { inner: UnsafeCell::new(None) },
        init: Cell::new(Some(|| 42)),
    };

    let value: &i32 = Lazy::force(&lazy);
}

#[test]
fn test_force_with_string_function() {
    struct TestFunction;

    let lazy = Lazy {
        cell: OnceCell { inner: UnsafeCell::new(None) },
        init: Cell::new(Some(|| String::from("Hello, world!"))),
    };

    let value: &String = Lazy::force(&lazy);
}

#[test]
fn test_force_with_custom_struct_function() {
    #[derive(Debug)]
    struct CustomStruct {
        value: i32,
    }

    struct TestFunction;

    let lazy = Lazy {
        cell: OnceCell { inner: UnsafeCell::new(None) },
        init: Cell::new(Some(|| CustomStruct { value: 99 })),
    };

    let value: &CustomStruct = Lazy::force(&lazy);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_on_poisoned_lazy() {
    struct TestFunction;

    let lazy = Lazy {
        cell: OnceCell { inner: UnsafeCell::new(None) },
        init: Cell::new(None), // Simulate a poisoned state.
    };

    let _value: &i32 = Lazy::force(&lazy);
}

#[test]
fn test_force_multiple_calls() {
    struct TestFunction;

    let lazy = Lazy {
        cell: OnceCell { inner: UnsafeCell::new(None) },
        init: Cell::new(Some(|| 100)),
    };

    let first_call: &i32 = Lazy::force(&lazy);
    let second_call: &i32 = Lazy::force(&lazy);
}

