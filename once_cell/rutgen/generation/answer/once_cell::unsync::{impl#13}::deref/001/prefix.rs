// Answer 0

#[test]
fn test_deref_valid_initialization() {
    let lazy: Lazy<i32, _> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(None),
        },
        init: Cell::new(Some(|| 42)),
    };
    let value = *lazy.deref();
}

#[test]
fn test_deref_multiple_initializations() {
    let lazy: Lazy<String, _> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(None),
        },
        init: Cell::new(Some(|| "Hello".to_string())),
    };
    let value = lazy.deref();
}

#[test]
#[should_panic]
fn test_deref_poisoned_lazy() {
    let lazy: Lazy<i32, _> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(None),
        },
        init: Cell::new(None),
    };
    let value = *lazy.deref();
}

#[test]
fn test_deref_uninitialized_once_cell() {
    let lazy: Lazy<f64, _> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(None),
        },
        init: Cell::new(Some(|| 3.14)),
    };
    let value = *lazy.deref();
}

#[test]
fn test_deref_empty_optional_init() {
    let lazy: Lazy<bool, _> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(None),
        },
        init: Cell::new(Some(|| true)),
    };
    let value = *lazy.deref();
}

