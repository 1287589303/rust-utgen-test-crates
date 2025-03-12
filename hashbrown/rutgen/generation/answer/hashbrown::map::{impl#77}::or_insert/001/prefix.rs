// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    struct HashMapBuilder;

    impl BuildHasher for HashMapBuilder {
        type Hasher = core::hash::BuildHasherDefault<core::hash::blake3::Hasher>;

        fn build_hasher(&self) -> Self::Hasher {
            core::hash::BuildHasherDefault::default()
        }
    }

    let mut map: hashbrown::HashMap<&str, i32> = hashbrown::HashMap::new();
    let key = "vacant_key";
    let default_value = 42;
    let entry = map.entry(key);

    if let hashbrown::hash_map::Entry::Vacant(vacant_entry) = entry {
        let value_ref = vacant_entry.or_insert(default_value);

        // Use the inserted value reference to ensure it works correctly
        *value_ref += 10;
    }
}

#[test]
fn test_or_insert_vacant_entry_with_collision_handling() {
    struct HashMapBuilder;

    impl BuildHasher for HashMapBuilder {
        type Hasher = core::hash::BuildHasherDefault<core::hash::blake3::Hasher>;

        fn build_hasher(&self) -> Self::Hasher {
            core::hash::BuildHasherDefault::default()
        }
    }

    let mut map: hashbrown::HashMap<&str, i32> = hashbrown::HashMap::new();
    map.insert("existing_key", 1);
    let key = "vacant_key"; // Ensure this key is not in the map
    let default_value = 23;
    let entry = map.entry(key);

    if let hashbrown::hash_map::Entry::Vacant(vacant_entry) = entry {
        let value_ref = vacant_entry.or_insert(default_value);
        
        // Ensure the value is inserted correctly
        *value_ref *= 2;
    }
}

