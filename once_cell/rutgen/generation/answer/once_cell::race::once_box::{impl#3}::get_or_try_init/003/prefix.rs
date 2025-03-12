// Answer 0

#[test]
fn test_get_or_try_init_concurrent_success() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    struct MyData {
        value: i32,
    }

    let once_box = Arc::new(OnceBox::new());
    let once_box_clone1 = Arc::clone(&once_box);
    let once_box_clone2 = Arc::clone(&once_box);

    let result = Mutex::new(None);

    let thread1 = thread::spawn(move || {
        let value = once_box_clone1.get_or_try_init(|| {
            Ok(Box::new(MyData { value: 42 }))
        });
        let mut res = result.lock().unwrap();
        *res = Some(value);
    });

    let thread2 = thread::spawn(move || {
        let value = once_box_clone2.get_or_try_init(|| {
            Ok(Box::new(MyData { value: 43 }))
        });
        let mut res = result.lock().unwrap();
        *res = Some(value);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    let res = result.lock().unwrap();

    let _ = &res; // Used to ensure the result is accessed
}

#[test]
fn test_get_or_try_init_concurrent_error_exchange() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    struct MyData {
        value: i32,
    }

    let once_box = Arc::new(OnceBox::new());
    let once_box_clone1 = Arc::clone(&once_box);
    let once_box_clone2 = Arc::clone(&once_box);

    let result = Mutex::new(None);

    let thread1 = thread::spawn(move || {
        let _ = once_box_clone1.get_or_try_init(|| {
            Ok(Box::new(MyData { value: 42 }))
        });
    });

    let thread2 = thread::spawn(move || {
        let value = once_box_clone2.get_or_try_init(|| {
            Err("error")
        });
        let mut res = result.lock().unwrap();
        *res = Some(value);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    let res = result.lock().unwrap();

    let _ = &res; // Used to ensure the result is accessed
}

