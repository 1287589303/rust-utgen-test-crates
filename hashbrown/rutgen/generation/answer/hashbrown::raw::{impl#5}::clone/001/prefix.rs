// Answer 0

#[test]
fn test_clone_with_valid_non_null_pointer() {
    struct TestStruct;
    let value = TestStruct;
    let layout = std::alloc::Layout::new::<TestStruct>();
    let ptr = unsafe { NonNull::new(std::alloc::alloc(layout)).unwrap() };
    unsafe {
        ptr.as_ptr().write(value);
    }
    let bucket = Bucket { ptr };
    let _cloned_bucket = bucket.clone();
}

#[test]
fn test_clone_with_invalid_non_null_pointer() {
    struct TestStruct;
    let _bucket = Bucket { ptr: NonNull::new(1 as *mut TestStruct).unwrap() };
    let _cloned_bucket = _bucket.clone();
}

#[test]
#[should_panic]
fn test_clone_with_null_pointer() {
    struct TestStruct;
    let _bucket = Bucket { ptr: NonNull::dangling() };
    let _cloned_bucket = _bucket.clone();
}

#[test]
#[should_panic]
fn test_clone_with_dangling_pointer() {
    struct TestStruct;
    let ptr = NonNull::new(unsafe { std::mem::transmute::<*mut TestStruct, *mut usize>(0xDEADBEEF as *mut TestStruct) }).unwrap();
    let _bucket = Bucket { ptr };
    let _cloned_bucket = _bucket.clone();
}

