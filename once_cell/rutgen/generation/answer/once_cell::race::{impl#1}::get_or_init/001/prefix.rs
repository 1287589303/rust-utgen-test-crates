// Answer 0

#[test]
fn test_get_or_init_single_thread_true() {
    let once_bool = OnceBool::new();
    let result = once_bool.get_or_init(|| true);
}

#[test]
fn test_get_or_init_single_thread_false() {
    let once_bool = OnceBool::new();
    let result = once_bool.get_or_init(|| false);
}

#[test]
fn test_get_or_init_multiple_threads_true() {
    let once_bool = OnceBool::new();
    let handles: Vec<_> = (0..10).map(|_| {
        let once_bool_clone = &once_bool;
        std::thread::spawn(move || {
            once_bool_clone.get_or_init(|| true);
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_get_or_init_multiple_threads_false() {
    let once_bool = OnceBool::new();
    let handles: Vec<_> = (0..10).map(|_| {
        let once_bool_clone = &once_bool;
        std::thread::spawn(move || {
            once_bool_clone.get_or_init(|| false);
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_get_or_init_mixed_threads() {
    let once_bool = OnceBool::new();
    let handles: Vec<_> = (0..5).map(|_| {
        let once_bool_clone = &once_bool;
        std::thread::spawn(move || {
            once_bool_clone.get_or_init(|| true);
        })
    }).chain((0..5).map(|_| {
        let once_bool_clone = &once_bool;
        std::thread::spawn(move || {
            once_bool_clone.get_or_init(|| false);
        })
    })).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

