// Answer 0

#[test]
fn test_or_insert_occupied_entry() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut entries: Entries<i32, String> = Entries::new();
    let key: i32 = 10;
    let value: String = String::from("value10");
    entries.insert(key, value.clone());

    let index = entries.get_index_of(&key).unwrap();
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData::<TestHasher>,
    };

    let raw_entry = RawEntryMut::Occupied(occupied_entry);

    let (mut k, mut v) = raw_entry.or_insert(20, String::from("default_value"));
    *k = 30;
    *v = String::from("new_value");
}

#[test]
fn test_or_insert_vacant_entry() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut entries: Entries<i32, String> = Entries::new();
    let key: i32 = 20;
    let value: String = String::from("value20");

    let vacant_entry = RawVacantEntryMut {
        map: RefMut::new(&mut entries),
        hash_builder: &TestHasher,
    };

    let raw_entry = RawEntryMut::Vacant(vacant_entry);

    let (mut k, mut v) = raw_entry.or_insert(key, value);
    *k = 40;
    *v = String::from("inserted_value");
}

