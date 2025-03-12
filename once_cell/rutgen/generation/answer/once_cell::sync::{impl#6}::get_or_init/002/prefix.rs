// Answer 0

#[test]
fn test_get_or_init_with_value() {
    struct IntWrapper(i32);
    let cell: OnceCell<IntWrapper> = OnceCell::new();
    let value = cell.get_or_init(|| IntWrapper(42));
}

#[test]
fn test_get_or_init_with_zero() {
    let cell: OnceCell<u32> = OnceCell::new();
    let value = cell.get_or_init(|| 0);
}

#[test]
fn test_get_or_init_with_large_number() {
    let cell: OnceCell<u64> = OnceCell::new();
    let value = cell.get_or_init(|| 1_000_000_000);
}

#[test]
fn test_get_or_init_with_float() {
    let cell: OnceCell<f64> = OnceCell::new();
    let value = cell.get_or_init(|| 3.14);
}

#[test]
fn test_get_or_init_concurrent_usage() {
    use std::thread;
    
    let cell: OnceCell<i32> = OnceCell::new();
    
    let handles: Vec<_> = (0..10).map(|_| {
        let cell_clone = &cell;
        thread::spawn(move || {
            cell_clone.get_or_init(|| 100);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

