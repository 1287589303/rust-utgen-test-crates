// Answer 0

#[test]
fn test_raw_entry_mut_index_vacant() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = MockHasher;
    let map = {
        let mut indices = Vec::new();
        indices.push(0);
        indices
    };

    let vacant_entry = RawVacantEntryMut {
        map: RefMut {
            indices,
        },
        hash_builder: &hasher,
    };

    let entry = RawEntryMut::Vacant(vacant_entry);
    let result_index = entry.index();
}

#[test]
fn test_raw_entry_mut_index_vacant_empty_map() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = MockHasher;
    let vacant_entry = RawVacantEntryMut {
        map: RefMut {
            indices: Vec::new(),
        },
        hash_builder: &hasher,
    };

    let entry = RawEntryMut::Vacant(vacant_entry);
    let result_index = entry.index();
}

#[test]
fn test_raw_entry_mut_index_vacant_multiple_entries() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = MockHasher;
    let vacant_entry = RawVacantEntryMut {
        map: RefMut {
            indices: vec![0, 1, 2],
        },
        hash_builder: &hasher,
    };

    let entry = RawEntryMut::Vacant(vacant_entry);
    let result_index = entry.index();
}

