// Answer 0

#[test]
fn test_pointer_as_usize_valid_pointer() {
    let value: i32 = 42;
    let ptr: *const i32 = &value;
    let result = ptr.as_usize();
}

#[test]
fn test_pointer_as_usize_null_pointer() {
    let ptr: *const i32 = std::ptr::null();
    let result = ptr.as_usize();
}

#[test]
fn test_pointer_as_usize_maximum_pointer() {
    let ptr: *const i32 = std::usize::MAX as *const i32;
    let result = ptr.as_usize();
}

#[test]
fn test_pointer_as_usize_smallest_address() {
    let ptr: *const i32 = 1 as *const i32;
    let result = ptr.as_usize();
}

