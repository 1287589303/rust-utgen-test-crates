// Answer 0

#[test]
fn test_swap_remove_entry_valid() {
    struct TestEntries {
        data: Vec<(HashValue, i32, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (HashValue, i32, String);
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

    let mut entries = TestEntries { data: vec![
        (HashValue::from(1), 10, "ten".to_string()),
        (HashValue::from(2), 20, "twenty".to_string())
    ]};

    let occupied_entry = OccupiedEntry::new(
        &mut entries,
        hash_table::OccupiedEntry::new(0)
    );

    let _result = occupied_entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_boundary() {
    struct TestEntries {
        data: Vec<(HashValue, i32, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (HashValue, i32, String);
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

    let mut entries = TestEntries { data: vec![
        (HashValue::from(3), 30, "thirty".to_string()),
        (HashValue::from(4), 40, "forty".to_string())
    ]};

    let occupied_entry_last = OccupiedEntry::new(
        &mut entries,
        hash_table::OccupiedEntry::new(1)
    );

    let _result_last = occupied_entry_last.swap_remove_entry();

    let occupied_entry_first = OccupiedEntry::new(
        &mut entries,
        hash_table::OccupiedEntry::new(0)
    );

    let _result_first = occupied_entry_first.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_non_empty() {
    struct TestEntries {
        data: Vec<(HashValue, i32, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (HashValue, i32, String);
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

    let mut entries = TestEntries { data: vec![
        (HashValue::from(5), 50, "fifty".to_string()),
        (HashValue::from(6), 60, "sixty".to_string()),
        (HashValue::from(7), 70, "seventy".to_string())
    ]};

    for index in 0..entries.data.len() {
        let occupied_entry = OccupiedEntry::new(
            &mut entries,
            hash_table::OccupiedEntry::new(index),
        );

        let _result = occupied_entry.swap_remove_entry();
    }
}

