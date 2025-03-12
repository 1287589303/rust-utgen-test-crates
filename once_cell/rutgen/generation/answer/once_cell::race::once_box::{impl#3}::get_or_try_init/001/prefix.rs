// Answer 0

#[test]
fn test_get_or_try_init_with_err() {
    use std::sync::Arc;
    use std::thread;

    struct DummyError;

    let once_box: Arc<OnceBox<u32>> = Arc::new(OnceBox::new());

    let f = || {
        // This function always returns an error
        Err(DummyError)
    };

    let threads: Vec<_> = (0..10).map(|_| {
        let once_box_clone = Arc::clone(&once_box);
        thread::spawn(move || {
            let _result: Result<&u32, DummyError> = once_box_clone.get_or_try_init(f);
        })
    }).collect();

    for thread in threads {
        let _ = thread.join();
    }
}

#[test]
fn test_get_or_try_init_with_none() {
    use std::sync::Arc;
    use std::thread;

    let once_box: Arc<OnceBox<u32>> = Arc::new(OnceBox::new());

    let f = || {
        // This function returns None wrapped in an Option
        None
    };

    let threads: Vec<_> = (0..10).map(|_| {
        let once_box_clone = Arc::clone(&once_box);
        thread::spawn(move || {
            let _result: Result<&u32, Option<()>> = once_box_clone.get_or_try_init(f);
        })
    }).collect();

    for thread in threads {
        let _ = thread.join();
    }
}

