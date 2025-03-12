// Answer 0

#[test]
fn test_deref_with_i32() {
    let value: i32 = 42;
    let mut guard = ScopeGuard {
        dropfn: |x: &mut i32| *x += 1,
        value,
    };
    let _ = &*guard; // Call deref
}

#[test]
fn test_deref_with_string() {
    let value: String = String::from("Hello");
    let mut guard = ScopeGuard {
        dropfn: |x: &mut String| x.push_str(", World!"),
        value,
    };
    let _ = &*guard; // Call deref
}

#[test]
fn test_deref_with_struct() {
    struct MyStruct {
        field: i32,
    }
    let value = MyStruct { field: 10 };
    let mut guard = ScopeGuard {
        dropfn: |x: &mut MyStruct| x.field += 5,
        value,
    };
    let _ = &*guard; // Call deref
}

#[test]
fn test_deref_with_option() {
    let value: Option<i32> = Some(7);
    let mut guard = ScopeGuard {
        dropfn: |x: &mut Option<i32>| *x = None,
        value,
    };
    let _ = &*guard; // Call deref
}

