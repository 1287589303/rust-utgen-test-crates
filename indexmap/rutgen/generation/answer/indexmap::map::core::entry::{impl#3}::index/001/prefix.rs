// Answer 0

#[test]
fn test_index_valid() {
    struct TestEntries {
        entries: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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

    let mut entries = TestEntries { entries: vec![1, 2, 3] };
    let occupied_index = hashbrown::hash_table::OccupiedEntry::from_index(0);
    let occupied_entry = OccupiedEntry::new(&mut entries, occupied_index);
    let result = occupied_entry.index();
}

#[test]
fn test_index_empty() {
    struct TestEntries {
        entries: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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

    let mut entries = TestEntries { entries: vec![] };
    let occupied_index = hashbrown::hash_table::OccupiedEntry::from_index(0);
    let occupied_entry = OccupiedEntry::new(&mut entries, occupied_index);
    let result = occupied_entry.index();
}

#[test]
#[should_panic] 
fn test_index_out_of_bounds() {
    struct TestEntries {
        entries: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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

    let mut entries = TestEntries { entries: vec![1, 2, 3] };
    let occupied_index = hashbrown::hash_table::OccupiedEntry::from_index(4); // out of bounds
    let occupied_entry = OccupiedEntry::new(&mut entries, occupied_index);
    let result = occupied_entry.index();
}

