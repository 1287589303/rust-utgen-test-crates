// Answer 0

#[test]
fn test_drop_with_init_state_and_null_pointer() {
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicU8, Ordering};

    const LAZY_STATE_INIT: u8 = 0;
    const LAZY_STATE_BUSY: u8 = 1;

    struct LazyTest {
        state: AtomicU8,
        create: Cell<Option<fn()>>,
        data: Cell<MaybeUninit<*mut u8>>,
    }

    let lazy = LazyTest {
        state: AtomicU8::new(LAZY_STATE_INIT),
        create: Cell::new(None),
        data: Cell::new(MaybeUninit::new(core::ptr::null_mut())),
    };

    // Call the drop function
    let _ = unsafe { &mut *(std::mem::transmute::<&LazyTest, *mut LazyTest>(&lazy)) };
}

#[test]
fn test_drop_with_busy_state_and_null_pointer() {
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicU8, Ordering};

    const LAZY_STATE_INIT: u8 = 0;
    const LAZY_STATE_BUSY: u8 = 1;

    struct LazyTest {
        state: AtomicU8,
        create: Cell<Option<fn()>>,
        data: Cell<MaybeUninit<*mut u8>>,
    }

    let lazy = LazyTest {
        state: AtomicU8::new(LAZY_STATE_BUSY),
        create: Cell::new(None),
        data: Cell::new(MaybeUninit::new(core::ptr::null_mut())),
    };

    // Call the drop function
    let _ = unsafe { &mut *(std::mem::transmute::<&LazyTest, *mut LazyTest>(&lazy)) };
}

