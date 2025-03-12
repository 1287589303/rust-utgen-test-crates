// Answer 0

#[test]
fn test_hash_valid_key_low_values() {
    let mut map = Utf8SuffixMap::new(1);
    let state_id = StateID(0);
    let key = Utf8SuffixKey {
        from: state_id,
        start: 0,
        end: 0,
    };
    let _ = map.hash(&key);
}

#[test]
fn test_hash_valid_key_high_values() {
    let mut map = Utf8SuffixMap::new(1);
    let state_id = StateID(0);
    let key = Utf8SuffixKey {
        from: state_id,
        start: 255,
        end: 255,
    };
    let _ = map.hash(&key);
}

#[test]
fn test_hash_key_mid_range() {
    let mut map = Utf8SuffixMap::new(1);
    let state_id = StateID(0);
    let key = Utf8SuffixKey {
        from: state_id,
        start: 128,
        end: 128,
    };
    let _ = map.hash(&key);
}

#[test]
fn test_hash_non_zero_length() {
    let mut map = Utf8SuffixMap::new(2);
    let state_id = StateID(1);
    let key = Utf8SuffixKey {
        from: state_id,
        start: 100,
        end: 200,
    };
    let _ = map.hash(&key);
}

#[test]
fn test_hash_large_state_id() {
    let mut map = Utf8SuffixMap::new(1);
    let state_id = StateID(u64::MAX as SmallIndex);
    let key = Utf8SuffixKey {
        from: state_id,
        start: 1,
        end: 2,
    };
    let _ = map.hash(&key);
}

