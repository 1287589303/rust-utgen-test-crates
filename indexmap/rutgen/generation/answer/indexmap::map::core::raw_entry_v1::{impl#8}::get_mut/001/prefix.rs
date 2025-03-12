// Answer 0

#[test]
fn test_get_mut_valid_index() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::from(1), key: 0, value: "Value 0".to_string() },
            Bucket { hash: HashValue::from(2), key: 1, value: "Value 1".to_string() },
        ],
    };

    let index = 0;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_raw(index),
        hash_builder: PhantomData,
    };

    let result = occupied_entry.get_mut();
}

#[test]
fn test_get_mut_boundaries() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::from(1), key: 0, value: "First".to_string() },
            Bucket { hash: HashValue::from(2), key: 1, value: "Second".to_string() },
        ],
    };

    let occupied_entry_0 = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_raw(0),
        hash_builder: PhantomData,
    };

    let result_0 = occupied_entry_0.get_mut();

    let occupied_entry_1 = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_raw(1),
        hash_builder: PhantomData,
    };

    let result_1 = occupied_entry_1.get_mut();
}

