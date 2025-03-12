// Answer 0

#[test]
fn test_wait_initialization_with_value() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    let cell = Arc::new(Mutex::new(OnceCell::new()));
    
    let thread_cell = Arc::clone(&cell);
    let t = thread::spawn(move || {
        let mut cell = thread_cell.lock().unwrap();
        cell.set(42).unwrap();
    });

    {
        let cell = cell.lock().unwrap();
        // Ensures that the cell is initialized before calling wait
        let _ = cell.get_or_init(|| 42);
    }

    let value: &u32 = cell.lock().unwrap().wait();
    assert_eq!(*value, 42);
    
    t.join().unwrap();
}

#[test]
fn test_wait_after_set() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let cell = Arc::new(Mutex::new(OnceCell::new()));
    let thread_cell = Arc::clone(&cell);
    let t = thread::spawn(move || {
        let mut cell = thread_cell.lock().unwrap();
        cell.set(99).unwrap();
    });

    {
        let cell = cell.lock().unwrap();
        let _ = cell.get_or_init(|| 99);
        // Now wait after the set
        let value: &u32 = cell.wait();
        assert_eq!(*value, 99);
    }
    
    t.join().unwrap();
}

#[test]
fn test_wait_no_preexisting_value() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let cell = Arc::new(Mutex::new(OnceCell::new()));
    let thread_cell = Arc::clone(&cell);
    let t = thread::spawn(move || {
        let mut cell = thread_cell.lock().unwrap();
        cell.set(27).unwrap();
    });

    {
        let cell = cell.lock().unwrap();
        // wait will block until the value is set
        let value: &u32 = cell.wait();
        assert_eq!(*value, 27);
    }

    t.join().unwrap();
}

