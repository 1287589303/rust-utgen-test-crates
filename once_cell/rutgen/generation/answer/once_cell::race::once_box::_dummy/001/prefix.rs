// Answer 0

#[test]
fn test_once_box_with_valid_pointer() {
    struct S(*mut ());
    unsafe impl Sync for S {}
    let ptr: *mut () = Box::into_raw(Box::new(42)) as *mut ();
    let once_box = once_cell::race::OnceBox::new(S(ptr));
    share(&once_box);
}

#[test]
fn test_once_box_with_null_pointer() {
    struct S(*mut ());
    unsafe impl Sync for S {}
    let once_box = once_cell::race::OnceBox::new(S(ptr::null_mut()));
    share(&once_box);
}

#[test]
fn test_once_box_with_another_valid_pointer() {
    struct S(*mut ());
    unsafe impl Sync for S {}
    let ptr: *mut () = Box::into_raw(Box::new("Hello")) as *mut ();
    let once_box = once_cell::race::OnceBox::new(S(ptr));
    share(&once_box);
}

#[test]
fn test_once_box_with_invalid_pointer() {
    struct S(*mut ());
    unsafe impl Sync for S {}
    let invalid_ptr: *mut () = 12345 as *mut (); // arbitrary invalid pointer
    let once_box = once_cell::race::OnceBox::new(S(invalid_ptr));
    share(&once_box);
}

#[test]
fn test_once_box_with_large_memory_allocation() {
    struct S(*mut ());
    unsafe impl Sync for S {}
    let size = 1_000_000; // large size
    let ptr: *mut () = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(size, 8).unwrap()) };
    let once_box = once_cell::race::OnceBox::new(S(ptr));
    share(&once_box);
    unsafe { alloc::alloc::dealloc(ptr, alloc::alloc::Layout::from_size_align(size, 8).unwrap()) }; // deallocate
}

