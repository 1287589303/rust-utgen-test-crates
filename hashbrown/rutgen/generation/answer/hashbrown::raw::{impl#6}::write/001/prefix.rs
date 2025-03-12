// Answer 0

#[test]
fn test_write_with_non_null_pointer() {
    struct TestType {
        data: i32,
    }
    
    let value = TestType { data: 42 };
    let ptr = Box::into_raw(Box::new(value));
    let non_null = NonNull::new(ptr).unwrap();
    let bucket = Bucket { ptr: non_null };

    unsafe {
        bucket.write(TestType { data: 100 });
    }
}

#[test]
fn test_write_zero_sized_type() {
    #[repr(C)]
    struct ZST;

    let ptr = Box::into_raw(Box::new(ZST));
    let non_null = NonNull::new(ptr).unwrap();
    let bucket = Bucket { ptr: non_null };

    unsafe {
        bucket.write(ZST);
    }
}

#[test]
fn test_write_with_different_data() {
    struct TestType {
        data: i32,
    }
    
    let value = TestType { data: 7 };
    let ptr = Box::into_raw(Box::new(value));
    let non_null = NonNull::new(ptr).unwrap();
    let bucket = Bucket { ptr: non_null };

    unsafe {
        bucket.write(TestType { data: 99 });
    }
}

#[test]
#[should_panic] // This test is to check behavior with uninitialized data
fn test_write_uninitialized_data() {
    struct Uninitialized;

    let ptr: *mut Uninitialized = std::ptr::null_mut();
    let non_null = NonNull::new(ptr).unwrap_or_else(|| panic!("Pointer is null"));
    let bucket = Bucket { ptr: non_null };

    unsafe {
        bucket.write(Uninitialized);
    }
}

