// Answer 0

#[test]
fn test_set_with_minimum_capacity_and_empty_key() {
    let mut map = Utf8BoundedMap::new(1);
    let key = vec![];
    let hash = map.hash(&key);
    let state_id = StateID(0);
    map.set(key, hash, state_id);
}

#[test]
fn test_set_with_small_capacity_and_single_transition() {
    let mut map = Utf8BoundedMap::new(10);
    let key = vec![Transition { start: 1, end: 1, next: StateID(1) }];
    let hash = map.hash(&key);
    let state_id = StateID(1);
    map.set(key.clone(), hash, state_id);
}

#[test]
fn test_set_with_capacity_at_boundary() {
    let mut map = Utf8BoundedMap::new(10);
    let key = vec![Transition { start: 0, end: 255, next: StateID(2) }];
    let hash = map.hash(&key);
    let state_id = StateID(2);
    map.set(key.clone(), hash, state_id);
}

#[test]
fn test_set_with_maximum_capacity() {
    let capacity = 1_000_000;
    let mut map = Utf8BoundedMap::new(capacity);
    let key = vec![Transition { start: 0, end: 0, next: StateID(3) }];
    let hash = map.hash(&key) % capacity;
    let state_id = StateID(3);
    map.set(key.clone(), hash, state_id);
}

#[test]
fn test_set_with_multiple_transitions() {
    let mut map = Utf8BoundedMap::new(10);
    let key = vec![
        Transition { start: 0, end: 1, next: StateID(4) },
        Transition { start: 2, end: 3, next: StateID(5) },
    ];
    let hash = map.hash(&key);
    let state_id = StateID(4);
    map.set(key, hash, state_id);
}

#[test]
fn test_set_with_full_range_of_transitions() {
    let mut map = Utf8BoundedMap::new(10);
    let key = (0..255)
        .map(|i| Transition { start: i, end: 255, next: StateID(i as u32) })
        .collect::<Vec<Transition>>();
    let hash = map.hash(&key);
    let state_id = StateID(255);
    map.set(key, hash, state_id);
}

