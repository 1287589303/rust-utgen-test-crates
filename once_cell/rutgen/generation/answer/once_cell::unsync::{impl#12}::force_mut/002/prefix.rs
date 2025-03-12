// Answer 0

#[test]
fn test_force_mut_with_uninitialized_cell_and_none_init() {
    struct TestInit;
    let mut lazy: Lazy<i32, fn()> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    let _result = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_uninitialized_cell_and_none_init_floating_point() {
    struct TestInit;
    let mut lazy: Lazy<f64, fn()> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    let _result = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_uninitialized_cell_and_none_init_string() {
    struct TestInit;
    let mut lazy: Lazy<String, fn()> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    let _result = Lazy::force_mut(&mut lazy);
}

