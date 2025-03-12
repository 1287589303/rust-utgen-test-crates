// Answer 0

#[test]
fn test_into_values_default_with_no_types() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let _value: IntoValues<(), (), MyAllocator> = IntoValues::default();
}

#[test]
fn test_into_values_default_with_some_types() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let _value: IntoValues<i32, String, MyAllocator> = IntoValues::default();
}

#[test]
fn test_into_values_default_with_edge_types() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let _value: IntoValues<isize, f32, MyAllocator> = IntoValues::default();
}

