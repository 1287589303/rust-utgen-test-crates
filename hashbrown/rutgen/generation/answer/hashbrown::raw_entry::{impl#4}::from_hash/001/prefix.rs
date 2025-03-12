// Answer 0

#[test]
fn test_from_hash_occupied_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    entry.insert(key, 100);
}

#[test]
fn test_from_hash_vacant_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "b";
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    entry.insert(key, 200);
}

#[test]
fn test_from_hash_empty_map() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "c";
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    entry.insert(key, 300);
}

