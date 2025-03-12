// Answer 0

#[test]
fn test_scope_guard_drop_fn_with_integer() {
    let mut value = 42;
    let mut drop_fn = |v: &mut i32| {
        *v += 1;
    };
    {
        let _guard = ScopeGuard { dropfn: drop_fn, value };
    }
}

#[test]
fn test_scope_guard_drop_fn_with_string() {
    let mut value = String::from("Hello");
    let mut drop_fn = |v: &mut String| {
        v.push_str(", World");
    };
    {
        let _guard = ScopeGuard { dropfn: drop_fn, value };
    }
}

#[test]
fn test_scope_guard_drop_fn_with_float() {
    let mut value = 3.14;
    let mut drop_fn = |v: &mut f64| {
        *v *= 2.0;
    };
    {
        let _guard = ScopeGuard { dropfn: drop_fn, value };
    }
}

#[test]
fn test_scope_guard_drop_fn_with_array() {
    let mut value = [1, 2, 3];
    let mut drop_fn = |v: &mut [i32; 3]| {
        v[0] += 1;
    };
    {
        let _guard = ScopeGuard { dropfn: drop_fn, value };
    }
}

#[test]
fn test_scope_guard_drop_fn_with_custom_struct() {
    struct TestStruct {
        data: i32,
    }
    let mut value = TestStruct { data: 10 };
    let mut drop_fn = |v: &mut TestStruct| {
        v.data += 5;
    };
    {
        let _guard = ScopeGuard { dropfn: drop_fn, value };
    }
}

