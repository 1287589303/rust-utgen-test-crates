// Answer 0

#[test]
fn test_memory_usage_zero_slots() {
    let cache = Cache {
        explicit_slots: vec![],
        explicit_slot_len: 0,
    };
    let _ = cache.memory_usage();
}

#[test]
fn test_memory_usage_one_slot() {
    let cache = Cache {
        explicit_slots: vec![None],
        explicit_slot_len: 1,
    };
    let _ = cache.memory_usage();
}

#[test]
fn test_memory_usage_multiple_slots() {
    let cache = Cache {
        explicit_slots: vec![None, Some(NonMaxUsize::new(1).unwrap()), None],
        explicit_slot_len: 3,
    };
    let _ = cache.memory_usage();
}

#[test]
fn test_memory_usage_large_slots() {
    let large_slot_count = 1000;
    let cache = Cache {
        explicit_slots: (0..large_slot_count).map(|_| Some(NonMaxUsize::new(1).unwrap())).collect(),
        explicit_slot_len: large_slot_count,
    };
    let _ = cache.memory_usage();
}

