// Answer 0

#[test]
fn test_drop_non_null_pointer() {
    use core::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use alloc::boxed::Box;

    struct TestStruct {
        value: i32,
    }

    let box_instance = Box::new(TestStruct { value: 42 });
    let inner_ptr: AtomicPtr<TestStruct> = AtomicPtr::new(Box::into_raw(box_instance));

    let once_box = OnceBox {
        inner: inner_ptr,
        ghost: PhantomData,
    };

    let _ = once_box; // This will trigger the Drop implementation
}

#[test]
fn test_drop_multiple_times() {
    use core::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use alloc::boxed::Box;

    struct TestStruct {
        value: i32,
    }

    let box_instance = Box::new(TestStruct { value: 84 });
    let inner_ptr: AtomicPtr<TestStruct> = AtomicPtr::new(Box::into_raw(box_instance));

    let once_box1 = OnceBox {
        inner: inner_ptr,
        ghost: PhantomData,
    };
    
    let once_box2 = OnceBox {
        inner: inner_ptr,
        ghost: PhantomData,
    };

    let _ = once_box1; // This will drop the first instance
    let _ = once_box2; // This will trigger the panic due to double drop, demonstrating invalid memory access
}

