// Answer 0

#[test]
fn test_thread_rng_basic() {
    let rng = thread_rng();
}

#[test]
fn test_thread_rng_multiple_instances() {
    let rng1 = thread_rng();
    let rng2 = thread_rng();
    let rng3 = thread_rng();
}

#[test]
fn test_thread_rng_clone() {
    let rng = thread_rng();
    let rng_clone = rng.clone();
}

#[test]
fn test_thread_rng_deprecated_function() {
    let rng = thread_rng();
} 

#[test]
fn test_thread_rng_thread_local_context() {
    let rng = thread_rng();
    let rng_from_another_thread = std::thread::spawn(|| thread_rng()).join().unwrap();
}

