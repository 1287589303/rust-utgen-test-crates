// Answer 0

#[test]
fn test_from_key_occupied_entry() {
    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct KeyType(u32);
    
    impl Hash for KeyType {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<KeyType> for KeyType {
        fn equivalent(&self, other: &KeyType) -> bool {
            self.0 == other.0
        }
    }
    
    let mut map = IndexMap::<KeyType, usize, MyHasher> {
        core: IndexMapCore::new(),
        hash_builder: MyHasher {},
    };
    
    map.insert(KeyType(5), 100); // Occupied entry
    
    let builder = RawEntryBuilderMut { map: &mut map };
    let key = KeyType(5);
    
    let entry = builder.from_key(&key);
}

#[test]
fn test_from_key_vacant_entry() {
    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct KeyType(u32);
    
    impl Hash for KeyType {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<KeyType> for KeyType {
        fn equivalent(&self, other: &KeyType) -> bool {
            self.0 == other.0
        }
    }
    
    let mut map = IndexMap::<KeyType, usize, MyHasher> {
        core: IndexMapCore::new(),
        hash_builder: MyHasher {},
    };
    
    let builder = RawEntryBuilderMut { map: &mut map };
    let key = KeyType(10); // Not inserted yet, should be vacant
    
    let entry = builder.from_key(&key);
}

#[test]
fn test_from_key_boundary_values() {
    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct KeyType(u32);
    
    impl Hash for KeyType {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<KeyType> for KeyType {
        fn equivalent(&self, other: &KeyType) -> bool {
            self.0 == other.0
        }
    }
    
    let mut map = IndexMap::<KeyType, usize, MyHasher> {
        core: IndexMapCore::new(),
        hash_builder: MyHasher {},
    };
    
    for i in 0..=u32::MAX {
        let occupied_key = KeyType(i);
        map.insert(occupied_key, i as usize); // Inserting at edge
        let builder = RawEntryBuilderMut { map: &mut map };
        
        let entry = builder.from_key(&KeyType(i)); // Testing occupied
        
        let key_not_in_map = KeyType(i + 1); // Next value should be vacant
        let vac_entry = builder.from_key(&key_not_in_map);
    }
}

