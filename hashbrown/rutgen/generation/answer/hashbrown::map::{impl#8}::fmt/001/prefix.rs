// Answer 0

#[test]
fn test_debug_empty_hashmap() {
    struct DebugKey;
    struct DebugValue;
    let map: HashMap<DebugKey, DebugValue> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::default(), Global);
    let _ = format!("{:?}", map);
}

#[test]
fn test_debug_single_entry_hashmap() {
    struct DebugKey;
    struct DebugValue;
    let mut map: HashMap<DebugKey, DebugValue> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::default(), Global);
    map.insert(DebugKey, DebugValue);
    let _ = format!("{:?}", map);
}

#[test]
fn test_debug_full_capacity_hashmap() {
    struct DebugKey;
    struct DebugValue;
    let mut map: HashMap<DebugKey, DebugValue> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::default(), Global);
    for _ in 0..5 {
        map.insert(DebugKey, DebugValue);
    }
    let _ = format!("{:?}", map);
}

