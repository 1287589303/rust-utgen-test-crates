// Answer 0

#[test]
fn test_drop_non_zero_sized() {
    struct TestData {
        value: i32,
    }

    let data = Box::new(TestData { value: 42 });
    let non_null_ptr = NonNull::new(Box::into_raw(data)).unwrap();
    let bucket = Bucket { ptr: non_null_ptr };
    
    unsafe {
        bucket.drop();
    }
}

#[test]
#[should_panic]
fn test_drop_non_initialised_memory() {
    struct TestData {
        value: i32,
    }

    let non_null_ptr = NonNull::new(0 as *mut TestData).unwrap();
    let bucket = Bucket { ptr: non_null_ptr };
    
    unsafe {
        bucket.drop();
    }
}

#[test]
fn test_drop_multiple_consecutive() {
    struct TestData {
        value: i32,
    }
    
    let data1 = Box::new(TestData { value: 1 });
    let data2 = Box::new(TestData { value: 2 });
    
    let non_null_ptr1 = NonNull::new(Box::into_raw(data1)).unwrap();
    let non_null_ptr2 = NonNull::new(Box::into_raw(data2)).unwrap();

    let bucket1 = Bucket { ptr: non_null_ptr1 };
    let bucket2 = Bucket { ptr: non_null_ptr2 };

    unsafe {
        bucket1.drop();
        bucket2.drop();
    }
}

