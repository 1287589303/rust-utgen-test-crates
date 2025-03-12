// Answer 0

#[test]
fn test_setup_search_zero() {
    let mut cache = Cache {
        explicit_slots: vec![None; 1],
        explicit_slot_len: 0,
    };
    cache.setup_search(0);
}

#[test]
fn test_setup_search_one() {
    let mut cache = Cache {
        explicit_slots: vec![None; 2],
        explicit_slot_len: 1,
    };
    cache.setup_search(1);
}

#[test]
fn test_setup_search_max_usize() {
    let mut cache = Cache {
        explicit_slots: vec![None; 3],
        explicit_slot_len: usize::MAX,
    };
    cache.setup_search(usize::MAX);
}

