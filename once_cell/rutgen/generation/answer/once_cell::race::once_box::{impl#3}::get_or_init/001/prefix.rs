// Answer 0

#[test]
fn test_get_or_init_with_uninitialized_inner() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    struct TestStruct {
        value: i32,
    }

    let once_box: OnceBox<TestStruct> = OnceBox::new(); 
    let f = || Box::new(TestStruct { value: 42 });

    let mut handles = vec![];
    for _ in 0..10 {
        let once_box_clone = Arc::new(once_box);
        let handle = thread::spawn({
            let once_box_clone = Arc::clone(&once_box_clone);
            move || {
                let _ = once_box_clone.get_or_init(f);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

#[test]
fn test_get_or_init_with_f_returning_different_values() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    struct TestStruct {
        value: i32,
    }

    let once_box: OnceBox<TestStruct> = OnceBox::new(); 
    let f = || {
        let value = rand::random::<i32>();
        Box::new(TestStruct { value })
    };

    let mut handles = vec![];
    for _ in 0..10 {
        let once_box_clone = Arc::new(once_box);
        let handle = thread::spawn({
            let once_box_clone = Arc::clone(&once_box_clone);
            move || {
                let _ = once_box_clone.get_or_init(f);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

