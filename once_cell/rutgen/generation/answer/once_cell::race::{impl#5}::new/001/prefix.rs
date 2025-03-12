// Answer 0

#[test]
fn test_once_ref_new_with_i32() {
    let _: OnceRef<i32> = OnceRef::new();
}

#[test]
fn test_once_ref_new_with_string() {
    let _: OnceRef<String> = OnceRef::new();
}

#[test]
fn test_once_ref_new_with_f64() {
    let _: OnceRef<f64> = OnceRef::new();
}

#[test]
fn test_once_ref_new_with_custom_struct() {
    struct MyStruct {
        value: i32,
    }
    let _: OnceRef<MyStruct> = OnceRef::new();
}

