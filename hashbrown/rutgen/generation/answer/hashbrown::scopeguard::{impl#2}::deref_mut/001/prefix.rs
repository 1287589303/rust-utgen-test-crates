// Answer 0

#[test]
fn test_deref_mut_i32() {
    let mut value = 42;
    let mut guard = ScopeGuard {
        dropfn: |v: &mut i32| *v += 1,
        value,
    };
    let _ref: &mut i32 = guard.deref_mut();
}

#[test]
fn test_deref_mut_string() {
    let mut value = String::from("Hello");
    let mut guard = ScopeGuard {
        dropfn: |v: &mut String| v.push_str(", World!"),
        value,
    };
    let _ref: &mut String = guard.deref_mut();
}

#[test]
fn test_deref_mut_custom_struct() {
    struct Custom {
        number: i32,
    }
    
    let mut value = Custom { number: 10 };
    let mut guard = ScopeGuard {
        dropfn: |v: &mut Custom| v.number += 5,
        value,
    };
    let _ref: &mut Custom = guard.deref_mut();
}

#[test]
fn test_deref_mut_empty_vec() {
    let mut value: Vec<i32> = Vec::new();
    let mut guard = ScopeGuard {
        dropfn: |v: &mut Vec<i32>| v.push(1),
        value,
    };
    let _ref: &mut Vec<i32> = guard.deref_mut();
}

