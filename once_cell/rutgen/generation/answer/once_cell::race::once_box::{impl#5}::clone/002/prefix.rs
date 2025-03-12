// Answer 0

#[test]
fn test_clone_none() {
    let once_box: OnceBox<i32> = OnceBox::new();
    let cloned_once_box = once_box.clone();
    let retrieved_value = cloned_once_box.get();
}

#[test]
fn test_clone_none_with_different_type() {
    let once_box: OnceBox<String> = OnceBox::new();
    let cloned_once_box = once_box.clone();
    let retrieved_value = cloned_once_box.get();
}

