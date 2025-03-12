// Answer 0

#[test]
fn test_move_index_to_lower_bound() {
    struct TestEntries {
        data: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![1, 2, 3] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(1));
    occupied_entry.move_index(0);
}

#[test]
fn test_move_index_to_upper_bound() {
    struct TestEntries {
        data: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![1, 2, 3] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0));
    occupied_entry.move_index(2);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_low() {
    struct TestEntries {
        data: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![1, 2, 3] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(1));
    occupied_entry.move_index(3);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_high() {
    struct TestEntries {
        data: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![1, 2, 3] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0));
    occupied_entry.move_index(4);
}

