// Answer 0

#[test]
fn test_shift_remove_valid_case() {
    struct TestEntries {
        data: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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

    let mut entries = TestEntries { data: vec![(0, "zero".to_string()), (1, "one".to_string()), (2, "two".to_string())] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(1),
        hash_builder: PhantomData,
    };
    
    let _ = occupied_entry.shift_remove();
}

#[test]
fn test_shift_remove_boundary_case_first_element() {
    struct TestEntries {
        data: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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

    let mut entries = TestEntries { data: vec![(0, "zero".to_string()), (1, "one".to_string()), (2, "two".to_string())] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(0),
        hash_builder: PhantomData,
    };
    
    let _ = occupied_entry.shift_remove();
}

#[test]
fn test_shift_remove_boundary_case_last_element() {
    struct TestEntries {
        data: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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

    let mut entries = TestEntries { data: vec![(0, "zero".to_string()), (1, "one".to_string()), (2, "two".to_string())] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(2),
        hash_builder: PhantomData,
    };
    
    let _ = occupied_entry.shift_remove();
}

#[test]
fn test_shift_remove_with_multiple_types() {
    struct TestEntries<T, U> {
        data: Vec<(T, U)>,
    }

    impl<T: Hash + Eq, U> Entries for TestEntries<T, U> {
        type Entry = (T, U);
        
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

    let mut entries = TestEntries { data: vec![("first", 1), ("second", 2), ("third", 3)] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(1),
        hash_builder: PhantomData,
    };

    let _ = occupied_entry.shift_remove();
}

