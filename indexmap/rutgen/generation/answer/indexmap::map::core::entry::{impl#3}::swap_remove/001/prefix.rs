// Answer 0

#[test]
fn test_swap_remove_non_empty() {
    struct TestEntries {
        data: Vec<(usize, usize)>,
    }
    
    impl Entries for TestEntries {
        type Entry = (usize, usize);
        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(0, 10), (1, 20), (2, 30)] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(1)); // Index 1 is occupied
    let value = occupied_entry.swap_remove();
}

#[test]
fn test_swap_remove_last_entry() {
    struct TestEntries {
        data: Vec<(usize, usize)>,
    }
    
    impl Entries for TestEntries {
        type Entry = (usize, usize);
        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(0, 10), (1, 20)] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(1)); // Index 1 is last
    let value = occupied_entry.swap_remove();
}

#[test]
fn test_swap_remove_single_entry() {
    struct TestEntries {
        data: Vec<(usize, usize)>,
    }
    
    impl Entries for TestEntries {
        type Entry = (usize, usize);
        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(0, 10)] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0)); // Only entry present
    let value = occupied_entry.swap_remove();
}

