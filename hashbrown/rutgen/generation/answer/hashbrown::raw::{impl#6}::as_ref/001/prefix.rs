// Answer 0

#[test]
fn test_as_ref_valid_pointer() {
    struct TestStruct {
        value: i32,
    }

    let value = TestStruct { value: 42 };
    let base_ptr = NonNull::from(&value);
    let bucket = unsafe { Bucket::from_base_index(base_ptr, 0) };

    unsafe {
        let result = bucket.as_ref();
        let deref_result = &*result;
    }
}

#[test]
fn test_as_ref_non_zero_sized() {
    struct NonZeroSize {
        value: i32,
    }

    let value = NonZeroSize { value: 99 };
    let base_ptr = NonNull::from(&value);
    let bucket = unsafe { Bucket::from_base_index(base_ptr, 0) };

    unsafe {
        let result = bucket.as_ref();
        let deref_result = &*result;
    }
}

#[test]
fn test_as_ref_with_multiple_offsets() {
    struct AnotherStruct {
        value: i32,
    }

    let value = AnotherStruct { value: 100 };
    let base_ptr = NonNull::from(&value);
    let bucket_zero = unsafe { Bucket::from_base_index(base_ptr, 0) };
    let bucket_one = unsafe { bucket_zero.next_n(1) };

    unsafe {
        let result_zero = bucket_zero.as_ref();
        let deref_result_zero = &*result_zero;

        let result_one = bucket_one.as_ref();
        let deref_result_one = &*result_one;
    }
}

#[test]
#[should_panic]
fn test_as_ref_zero_sized() {
    struct ZeroSized;

    let value = ZeroSized;
    let base_ptr = NonNull::from(&value);
    let bucket = unsafe { Bucket::from_base_index(base_ptr, 0) };

    unsafe {
        let result = bucket.as_ref();
        let deref_result = &*result;
    }
}

