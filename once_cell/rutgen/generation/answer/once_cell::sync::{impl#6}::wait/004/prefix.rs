// Answer 0

#[test]
fn test_wait_initializes_value() {
    use std::sync::Arc;
    use std::thread;
    use once_cell::OnceCell;

    let cell = Arc::new(OnceCell::new());
    let cell_clone = Arc::clone(&cell);
    
    let t = thread::spawn(move || {
        cell_clone.set(42).unwrap();
    });

    // At this point, the cell is not initialized
    let value: &u32 = cell.wait();

    // The value should now be initialized after calling `wait()`
    assert_eq!(*value, 42);
    t.join().unwrap();
}

#[test]
fn test_wait_with_multi_thread_initializes_value() {
    use std::sync::Arc;
    use std::thread;
    use once_cell::OnceCell;

    let cell = Arc::new(OnceCell::new());
    
    let threads: Vec<_> = (0..10).map(|i| {
        let cell_clone = Arc::clone(&cell);
        thread::spawn(move || {
            cell_clone.set(i * 10).unwrap();
        })
    }).collect();
    
    let value: &u32 = cell.wait();

    // The last thread to set should have set the value, 90 in this case.
    assert_eq!(*value, 90);
    
    for t in threads {
        t.join().unwrap();
    }
}

#[test]
#[should_panic]
fn test_wait_panics_if_not_set() {
    use std::sync::Arc;
    use std::thread;
    use once_cell::OnceCell;

    let cell = Arc::new(OnceCell::new());

    // Manually drop to simulate no value being set
    let _ = cell.wait(); // This should panic since we expect no value to be present 
}

