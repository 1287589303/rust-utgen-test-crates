// Answer 0

#[test]
fn test_remove_entry_valid_case() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, usize);
        
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

    let mut map = TestEntries { entries: vec![(0, 10), (1, 20)] };
    let index = hashbrown::hash_table::OccupiedEntry::new(&mut map.entries, 0);
    let entry = OccupiedEntry::new(&mut map, index);
    let (_key, _value) = entry.remove_entry();
}

#[test]
fn test_remove_entry_at_last_index() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, usize);
        
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

    let mut map = TestEntries { entries: vec![(0, 10), (1, 20)] };
    let index = hashbrown::hash_table::OccupiedEntry::new(&mut map.entries, 1);
    let (_key, _value) = entry.remove_entry();
}

#[test]
#[should_panic]
fn test_remove_entry_empty_map() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, usize);
        
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

    let mut map = TestEntries { entries: vec![] };
    let index = hashbrown::hash_table::OccupiedEntry::new(&mut map.entries, 0);
    let entry = OccupiedEntry::new(&mut map, index);
    entry.remove_entry();
}

