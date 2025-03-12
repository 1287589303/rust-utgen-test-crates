// Answer 0

#[test]
fn test_force_mut_with_empty_cell_and_none_initializer() {
    struct TestLazy;

    impl TestLazy {
        fn new() -> Lazy<i32, fn() -> i32> {
            Lazy {
                cell: OnceCell::new(),
                init: Cell::new(None),
            }
        }
    }

    let mut lazy = TestLazy::new();
    let result = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_empty_cell_and_none_initializer_after_set() {
    struct TestLazy;

    impl TestLazy {
        fn new() -> Lazy<i32, fn() -> i32> {
            Lazy {
                cell: OnceCell::new(),
                init: Cell::new(None),
            }
        }
    }

    let mut lazy = TestLazy::new();
    lazy.init.set(None);
    let result = Lazy::force_mut(&mut lazy);
}

