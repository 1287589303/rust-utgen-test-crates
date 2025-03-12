// Answer 0

#[test]
fn test_into_mut_valid_index() {
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
            Bucket { hash: HashValue::default(), key: 1, value: "One".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "Two".to_string() },
        ],
    };

    let index = 1; // Valid index
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut entries.entries, index));
    let mut_value: &mut String = occupied_entry.into_mut();
}

#[test]
fn test_into_mut_boundary_start() {
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
            Bucket { hash: HashValue::default(), key: 1, value: "One".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "Two".to_string() },
        ],
    };

    let index = 0; // Boundary index
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut entries.entries, index));
    let mut_value: &mut String = occupied_entry.into_mut();
}

#[test]
fn test_into_mut_boundary_end() {
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
            Bucket { hash: HashValue::default(), key: 1, value: "One".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "Two".to_string() },
        ],
    };

    let index = 1; // Boundary index
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut entries.entries, index));
    let mut_value: &mut String = occupied_entry.into_mut();
}

