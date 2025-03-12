// Answer 0

#[test]
fn test_swap_indices_valid() {
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
    
    let mut entries = TestEntries {
        data: vec![(0, 1), (2, 3), (4, 5)],
    };
    let occupied_entry_index = 1; // valid index
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(occupied_entry_index));

    occupied_entry.swap_indices(0);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds() {
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

    let mut entries = TestEntries {
        data: vec![(0, 1), (2, 3), (4, 5)],
    };
    let occupied_entry_index = 1; // valid index
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(occupied_entry_index));

    occupied_entry.swap_indices(3); // out of bounds
}

#[test]
fn test_swap_indices_same_index() {
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

    let mut entries = TestEntries {
        data: vec![(0, 1), (2, 3), (4, 5)],
    };
    let occupied_entry_index = 1; // valid index
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(occupied_entry_index));

    occupied_entry.swap_indices(1); // swapping with itself
}

