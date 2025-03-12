// Answer 0

#[test]
fn test_lazy_get_success() {
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicU8, Ordering};
    
    const LAZY_STATE_INIT: u8 = 0;
    const LAZY_STATE_BUSY: u8 = 1;
    const LAZY_STATE_DONE: u8 = 2;

    struct TestData {
        value: u32,
    }

    let state = AtomicU8::new(LAZY_STATE_INIT);
    let create = Cell::new(Some(|| {
        TestData { value: 42 }
    }));
    let data = Cell::new(MaybeUninit::uninit());

    let lazy = Lazy::<TestData, fn() -> TestData>(lazy::Lazy { state, create, data });

    let result = Lazy::get(&lazy);
}

#[test]
#[should_panic]
fn test_lazy_get_panic_in_initializer() {
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicU8, Ordering};
    
    const LAZY_STATE_INIT: u8 = 0;
    const LAZY_STATE_BUSY: u8 = 1;
    const LAZY_STATE_DONE: u8 = 2;

    struct TestData {
        value: u32,
    }

    let state = AtomicU8::new(LAZY_STATE_INIT);
    let create = Cell::new(Some(|| {
        panic!("Initializer panic");
    }));
    let data = Cell::new(MaybeUninit::uninit());

    let lazy = Lazy::<TestData, fn() -> TestData>(lazy::Lazy { state, create, data });

    let _result = Lazy::get(&lazy);
}

#[test]
fn test_lazy_get_thread_safety() {
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicU8, Ordering};
    use std::thread;

    const LAZY_STATE_INIT: u8 = 0;
    const LAZY_STATE_BUSY: u8 = 1;
    const LAZY_STATE_DONE: u8 = 2;

    struct TestData {
        value: u32,
    }

    let state = AtomicU8::new(LAZY_STATE_INIT);
    let create = Cell::new(Some(|| {
        TestData { value: 42 }
    }));
    let data = Cell::new(MaybeUninit::uninit());

    let lazy = Lazy::<TestData, fn() -> TestData>(lazy::Lazy { state, create, data });

    let handles: Vec<_> = (0..10).map(|_| {
        let lazy_clone = lazy.clone();
        thread::spawn(move || {
            let _result = Lazy::get(&lazy_clone);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

