// Answer 0

#[test]
fn test_key_mut_valid_case() {
    struct DummyKey;
    struct DummyValue;

    let mut map = IndexMapCore::<DummyKey, DummyValue>::new();
    let key = DummyKey;
    let value = DummyValue;

    map.insert(key, value);
    let index = 0;
    
    let mut entry = IndexedEntry::new(&mut map, index);
    let key_mut = entry.key_mut();
}

#[test]
fn test_key_mut_boundary_case() {
    struct DummyKey;
    struct DummyValue;

    let mut map = IndexMapCore::<DummyKey, DummyValue>::new();
    let key1 = DummyKey;
    let value1 = DummyValue;

    map.insert(key1, value1);
    let index = 0;

    let mut entry = IndexedEntry::new(&mut map, index);
    let key_mut = entry.key_mut();
}

#[test]
#[should_panic]
fn test_key_mut_empty_map() {
    struct DummyKey;
    struct DummyValue;

    let mut map = IndexMapCore::<DummyKey, DummyValue>::new();
    let index = 0;

    let mut entry = IndexedEntry::new(&mut map, index);
    let key_mut = entry.key_mut();
}

#[test]
#[should_panic]
fn test_key_mut_invalid_index() {
    struct DummyKey;
    struct DummyValue;

    let mut map = IndexMapCore::<DummyKey, DummyValue>::new();
    let key = DummyKey;
    let value = DummyValue;

    map.insert(key, value);
    let index = 1; // Invalid index

    let mut entry = IndexedEntry::new(&mut map, index);
    let key_mut = entry.key_mut();
}

