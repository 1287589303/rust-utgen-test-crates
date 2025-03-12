// Answer 0

#[test]
fn test_move_index_valid_case() {
    struct TestEntries {
        data: Vec<(usize, usize)>, // Simulating entries as tuple pairs
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

    let mut entries = TestEntries { data: vec![(0, 0), (1, 1), (2, 2)] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(occupied_entry_index), // a valid index
        hash_builder: PhantomData,
    };

    occupied_entry.move_index(1); // moving to a valid index within bounds
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_lower() {
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

    let mut entries = TestEntries { data: vec![(0, 0), (1, 1), (2, 2)] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(1), // valid index
        hash_builder: PhantomData,
    };

    occupied_entry.move_index(!0); // setting 'to' to an out-of-bounds index
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_upper() {
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

    let mut entries = TestEntries { data: vec![(0, 0), (1, 1), (2, 2)] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(1), // valid index
        hash_builder: PhantomData,
    };

    occupied_entry.move_index(3); // setting 'to' to an out-of-bounds index
}

