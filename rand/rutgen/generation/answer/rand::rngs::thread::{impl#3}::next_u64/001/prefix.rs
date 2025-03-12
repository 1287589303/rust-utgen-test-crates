// Answer 0

#[test]
fn test_next_u64_valid_mutable_reference() {
    use std::sync::Arc;
    use std::thread;

    struct TestCore;
    struct TestOsRng;

    impl RngCore for TestOsRng {
        fn next_u32(&mut self) -> u32 {
            42
        }
        fn next_u64(&mut self) -> u64 {
            42
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(42);
        }
    }

    impl RngCore for TestCore {
        fn next_u32(&mut self) -> u32 {
            42
        }
        fn next_u64(&mut self) -> u64 {
            42
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(42);
        }
    }

    let rng = ReseedingRng(TestCore);
    let thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(rng)),
    };

    let thread_rng_arc = Arc::new(thread_rng);
    let mut handles = vec![];

    for _ in 0..10 {
        let thread_rng_clone = Arc::clone(&thread_rng_arc);
        let handle = thread::spawn(move || {
            let mut rng_ref = unsafe { &mut *thread_rng_clone.rng.get() };
            rng_ref.next_u64();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_next_u64_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    struct TestCore;
    struct TestOsRng;

    impl RngCore for TestOsRng {
        fn next_u32(&mut self) -> u32 {
            42
        }
        fn next_u64(&mut self) -> u64 {
            42
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(42);
        }
    }

    impl RngCore for TestCore {
        fn next_u32(&mut self) -> u32 {
            42
        }
        fn next_u64(&mut self) -> u64 {
            42
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(42);
        }
    }

    let rng = ReseedingRng(TestCore);
    let thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(rng)),
    };

    let thread_rng_arc = Arc::new(thread_rng);
    let mut handles = vec![];

    for _ in 0..5 {
        let thread_rng_clone = Arc::clone(&thread_rng_arc);
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                let mut rng_ref = unsafe { &mut *thread_rng_clone.rng.get() };
                rng_ref.next_u64();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

