// Answer 0

#[test]
fn test_force_fallback_once() {
    force_fallback();
}

#[test]
fn test_force_fallback_concurrent() {
    let handles: Vec<_> = (0..10).map(|_| {
        std::thread::spawn(|| {
            force_fallback();
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

