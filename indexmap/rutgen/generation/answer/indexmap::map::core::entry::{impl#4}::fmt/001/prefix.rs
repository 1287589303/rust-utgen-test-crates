// Answer 0

#[test]
fn test_fmt_with_valid_entries() {
    struct TestEntries {
        entries: Vec<(String, i32)>,
    }

    impl Entries for TestEntries {
        type Entry = (String, i32);
        
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

    let mut test_entries = TestEntries {
        entries: vec![(String::from("key1"), 42)],
    };

    let occupied_index = hashbrown::hash_table::OccupiedEntry::new(&mut test_entries.entries, 0);
    let occupied_entry = OccupiedEntry::new(&mut test_entries, occupied_index);

    let mut formatter = fmt::Formatter::new();
    occupied_entry.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_multiple_entries() {
    struct TestEntries {
        entries: Vec<(String, i32)>,
    }

    impl Entries for TestEntries {
        type Entry = (String, i32);
        
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

    let mut test_entries = TestEntries {
        entries: vec![
            (String::from("key1"), 1),
            (String::from("key2"), 2),
        ],
    };

    let occupied_index = hashbrown::hash_table::OccupiedEntry::new(&mut test_entries.entries, 1);
    let occupied_entry = OccupiedEntry::new(&mut test_entries, occupied_index);

    let mut formatter = fmt::Formatter::new();
    occupied_entry.fmt(&mut formatter);
}

