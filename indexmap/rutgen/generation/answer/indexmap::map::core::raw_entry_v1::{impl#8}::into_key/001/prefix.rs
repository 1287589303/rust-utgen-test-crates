// Answer 0

#[test]
fn test_into_key_valid_case() {
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
            Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
            Bucket { hash: HashValue::default(), key: 3, value: "three".to_string() },
        ],
    };

    let index = 1; // Valid index for an entry
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(&mut entries.entries, index),
        hash_builder: PhantomData,
    };

    let key_mut = occupied_entry.into_key();
}

#[test]
fn test_into_key_boundary_case_zero() {
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
            Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
        ],
    };

    let index = 0; // Boundary index
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(&mut entries.entries, index),
        hash_builder: PhantomData,
    };

    let key_mut = occupied_entry.into_key();
}

#[test]
fn test_into_key_boundary_case_last() {
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
            Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
        ],
    };

    let index = 1; // Boundary index (last)
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(&mut entries.entries, index),
        hash_builder: PhantomData,
    };

    let key_mut = occupied_entry.into_key();
}

