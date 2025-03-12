// Answer 0

#[test]
fn test_as_mut_valid_pointer() {
    struct MyType {
        value: i32,
    }

    let mut value = MyType { value: 42 };
    let non_null_ptr = NonNull::new(&mut value).unwrap();
    let bucket = Bucket { ptr: non_null_ptr };

    unsafe {
        let result: &mut MyType = bucket.as_mut();
    }
}

#[test]
fn test_as_mut_non_zero_sized() {
    struct NonZeroSized {
        a: u8,
        b: u16,
    }

    let mut instance = NonZeroSized { a: 1, b: 2 };
    let non_null_instance = NonNull::new(&mut instance).unwrap();
    let bucket = Bucket { ptr: non_null_instance };

    unsafe {
        let result: &mut NonZeroSized = bucket.as_mut();
    }
}

#[test]
fn test_as_mut_multiple_instances() {
    struct AnotherType {
        data: [i32; 5],
    }

    let mut instance_a = AnotherType { data: [1, 2, 3, 4, 5] };
    let mut instance_b = AnotherType { data: [6, 7, 8, 9, 10] };
    let non_null_a = NonNull::new(&mut instance_a).unwrap();
    let non_null_b = NonNull::new(&mut instance_b).unwrap();

    let bucket_a = Bucket { ptr: non_null_a };
    let bucket_b = Bucket { ptr: non_null_b };

    unsafe {
        let result_a: &mut AnotherType = bucket_a.as_mut();
        let result_b: &mut AnotherType = bucket_b.as_mut();
    }
}

#[test]
#[should_panic]
fn test_as_mut_zero_sized_type() {
    struct ZeroSized;

    let mut instance = ZeroSized;
    let non_null_instance = NonNull::new(&mut instance).unwrap();
    let bucket = Bucket { ptr: non_null_instance };

    unsafe {
        let result: &mut ZeroSized = bucket.as_mut(); // This should panic.
    }
}

