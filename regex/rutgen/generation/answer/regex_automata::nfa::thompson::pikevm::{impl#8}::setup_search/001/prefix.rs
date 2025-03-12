// Answer 0

#[test]
fn test_setup_search_zero_slots() {
    let mut cache = Cache::new(&PikeVM {});
    cache.setup_search(0);
}

#[test]
fn test_setup_search_one_slot() {
    let mut cache = Cache::new(&PikeVM {});
    cache.setup_search(1);
}

#[test]
fn test_setup_search_two_slots() {
    let mut cache = Cache::new(&PikeVM {});
    cache.setup_search(2);
}

#[test]
fn test_setup_search_max_slots() {
    let mut cache = Cache::new(&PikeVM {});
    cache.setup_search(u32::MAX as usize);
}

