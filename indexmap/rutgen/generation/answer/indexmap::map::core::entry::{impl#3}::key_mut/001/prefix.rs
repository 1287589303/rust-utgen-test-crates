// Answer 0

#[test]
fn test_key_mut_valid() {
    struct TestEntries {
        data: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
        data: vec![
            Bucket { hash: HashValue::new(0), key: 1, value: 10 },
            Bucket { hash: HashValue::new(1), key: 2, value: 20 },
        ],
    };
    
    let index = 0; // within bounds
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(index));
    let key_mut_ref = occupied_entry.key_mut();

    // Implicitly checks if the mutable reference was successfully retrieved
    *key_mut_ref = 100; 
}

#[test]
fn test_key_mut_at_last_index() {
    struct TestEntries {
        data: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
        data: vec![
            Bucket { hash: HashValue::new(0), key: 1, value: 10 },
            Bucket { hash: HashValue::new(1), key: 2, value: 20 },
        ],
    };

    let index = 1; // last index
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(index));
    let key_mut_ref = occupied_entry.key_mut();

    // Implicitly checks if the mutable reference was successfully retrieved
    *key_mut_ref = 200; 
}

