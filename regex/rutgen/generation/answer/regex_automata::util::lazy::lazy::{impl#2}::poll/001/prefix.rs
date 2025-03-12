// Answer 0

#[test]
fn test_poll_with_uninitialized_data() {
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicU8, Ordering};
    
    struct Lazy<T, F> {
        state: AtomicU8,
        create: Cell<Option<F>>,
        data: Cell<MaybeUninit<T>>,
    }

    impl<T, F: FnOnce() -> T> Lazy<T, F> {
        pub(super) fn new() -> Self {
            Self {
                state: AtomicU8::new(0),
                create: Cell::new(None),
                data: Cell::new(MaybeUninit::uninit()),
            }
        }

        fn poll(&self) -> Option<&T> {
            let ptr = self.data.as_ptr(); // Correctly get the pointer
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }
    }

    let lazy_instance: Lazy<i32, fn() -> i32> = Lazy::new();
    let result = lazy_instance.poll();
}

#[test]
fn test_poll_with_uninitialized_data_f32() {
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicU8, Ordering};
    
    struct Lazy<T, F> {
        state: AtomicU8,
        create: Cell<Option<F>>,
        data: Cell<MaybeUninit<T>>,
    }

    impl<T, F: FnOnce() -> T> Lazy<T, F> {
        pub(super) fn new() -> Self {
            Self {
                state: AtomicU8::new(0),
                create: Cell::new(None),
                data: Cell::new(MaybeUninit::uninit()),
            }
        }

        fn poll(&self) -> Option<&T> {
            let ptr = self.data.as_ptr(); // Correctly get the pointer
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }
    }

    let lazy_instance: Lazy<f32, fn() -> f32> = Lazy::new();
    let result = lazy_instance.poll();
}

