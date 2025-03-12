// Answer 0

#[test]
fn test_swap_remove_entry_valid_removal() {
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
    let index = 1; // Ensure the index is valid
    let occupied_entry = OccupiedEntry { entries: &mut entries, index: hash_table::OccupiedEntry::new(index, &mut entries.data) };
    
    let removed_entry = occupied_entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_last_element() {
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

    let mut entries = TestEntries { data: vec![(0, 10)] }; // Only one element
    let occupied_entry = OccupiedEntry { entries: &mut entries, index: hash_table::OccupiedEntry::new(0, &mut entries.data) };
    
    let removed_entry = occupied_entry.swap_remove_entry();
}

#[test]
#[should_panic]
fn test_swap_remove_entry_invalid_index() {
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
    let index = 2; // Invalid index
    let occupied_entry = OccupiedEntry { entries: &mut entries, index: hash_table::OccupiedEntry::new(index, &mut entries.data) };
    
    let removed_entry = occupied_entry.swap_remove_entry();
}

