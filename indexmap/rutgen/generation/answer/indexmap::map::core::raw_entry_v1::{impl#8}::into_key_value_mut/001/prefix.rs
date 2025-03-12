// Answer 0

#[test]
fn test_into_key_value_mut_valid() {
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

    let mut test_entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "One".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "Two".to_string() },
        ],
    };

    let occupied_index = 0; // valid index as we have at least one entry
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied(occupied_index),
        hash_builder: PhantomData,
    };

    let (key, value) = entry.into_key_value_mut();
}

#[test]
fn test_into_key_value_mut_boundary() {
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

    let mut test_entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 3, value: "Three".to_string() },
        ],
    };

    let occupied_index = 0; // testing boundary condition with the last valid index
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied(occupied_index),
        hash_builder: PhantomData,
    };

    let (key, value) = entry.into_key_value_mut();
}

#[test]
fn test_into_key_value_mut_multiple_entries() {
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

    let mut test_entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 4, value: "Four".to_string() },
            Bucket { hash: HashValue::default(), key: 5, value: "Five".to_string() },
            Bucket { hash: HashValue::default(), key: 6, value: "Six".to_string() },
        ],
    };

    let occupied_index = 1; // valid index in the middle of multiple entries
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied(occupied_index),
        hash_builder: PhantomData,
    };

    let (key, value) = entry.into_key_value_mut();
}

