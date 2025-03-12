// Answer 0

#[test]
fn test_swap_indices_valid_swap() {
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
            where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }
    
    let mut entries = TestEntries { data: vec![(0, 1), (2, 3), (4, 5)] };
    let index_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(1),
        hash_builder: PhantomData,
    };
    index_entry.swap_indices(2);
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
            where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(0, 1), (2, 3), (4, 5)] };
    let index_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(1),
        hash_builder: PhantomData,
    };
    index_entry.swap_indices(1);
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
            where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(0, 1), (2, 3), (4, 5)] };
    let index_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    index_entry.swap_indices(3);
}

