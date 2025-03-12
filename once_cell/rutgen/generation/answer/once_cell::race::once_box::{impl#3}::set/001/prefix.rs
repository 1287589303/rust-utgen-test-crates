// Answer 0

#[test]
fn test_set_when_full() {
    struct TestStruct {
        value: i32,
    }
    
    let once_box = OnceBox::with_value(Box::new(TestStruct { value: 42 }));

    let new_value = Box::new(TestStruct { value: 99 });
    let result = once_box.set(new_value);
}

#[test]
fn test_set_with_different_type() {
    struct TestStruct {
        value: i32,
    }
    
    let once_box = OnceBox::with_value(Box::new(TestStruct { value: 42 }));

    let new_value = Box::new(TestStruct { value: 50 });
    let result = once_box.set(new_value);
}

#[test]
fn test_set_with_same_value() {
    struct TestStruct {
        value: i32,
    }
    
    let once_box = OnceBox::with_value(Box::new(TestStruct { value: 42 }));

    let same_value = Box::new(TestStruct { value: 42 });
    let result = once_box.set(same_value);
}

