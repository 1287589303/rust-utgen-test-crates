// Answer 0

#[test]
fn test_shift_insert_zero_index() {
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hash_builder = CustomHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: &mut map, hash_builder: &hash_builder };

    let key = "key1";
    let value = "value1";
    let result = entry.shift_insert(0, key, value);
}

#[test]
fn test_shift_insert_middle_index() {
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hash_builder = CustomHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: &mut map, hash_builder: &hash_builder };

    let key = "key2";
    let value = "value2";
    let result = entry.shift_insert(1, key, value);
}

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds() {
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hash_builder = CustomHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: &mut map, hash_builder: &hash_builder };

    let key = "key3";
    let value = "value3";
    let result = entry.shift_insert(10, key, value);
}

#[test]
fn test_shift_insert_at_current_size() {
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hash_builder = CustomHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: &mut map, hash_builder: &hash_builder };

    let key = "key4";
    let value = "value4";
    let result = entry.shift_insert(entries.len(), key, value);
}

