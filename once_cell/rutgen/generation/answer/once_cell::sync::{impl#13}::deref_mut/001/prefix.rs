// Answer 0

#[test]
fn test_deref_mut_initialized_with_value() {
    use std::cell::Cell;

    struct TestFn;

    impl FnOnce<()> for TestFn {
        type Output = i32;

        extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
            42
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(TestFn)),
    };

    let value = lazy.deref_mut();
}

#[test]
fn test_deref_mut_initialized_without_value() {
    struct TestFn;

    impl FnOnce<()> for TestFn {
        type Output = i32;

        extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
            42
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(TestFn)),
    };

    let value = lazy.deref_mut();
}

#[test]
#[should_panic]
fn test_deref_mut_poisoned() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };

    let value = lazy.deref_mut();
}

#[test]
fn test_deref_mut_uninitialized() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 100)),
    };

    let value = lazy.deref_mut();
}

