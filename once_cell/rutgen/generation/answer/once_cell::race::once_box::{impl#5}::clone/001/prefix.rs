// Answer 0

#[test]
fn test_clone_with_value() {
    struct TestStruct {
        data: i32,
    }
    
    let initial_value = Box::new(TestStruct { data: 42 });
    let once_box = OnceBox::with_value(initial_value);
    let cloned_box = once_box.clone();
    
    let original_value = once_box.get().unwrap();
    let cloned_value = cloned_box.get().unwrap();
    
    assert_eq!(original_value.data, cloned_value.data);
}

#[test]
fn test_clone_empty_once_box() {
    let once_box: OnceBox<TestStruct> = OnceBox::new();
    let cloned_box = once_box.clone();
    
    assert!(once_box.get().is_none());
    assert!(cloned_box.get().is_none());
}

#[test]
fn test_clone_after_set() {
    struct TestStruct {
        data: i32,
    }

    let once_box = OnceBox::new();
    let new_value = Box::new(TestStruct { data: 100 });
    once_box.set(new_value).expect("Expected to set value");
    
    let cloned_box = once_box.clone();
    
    assert!(once_box.get().is_some());
    assert!(cloned_box.get().is_some());
}

