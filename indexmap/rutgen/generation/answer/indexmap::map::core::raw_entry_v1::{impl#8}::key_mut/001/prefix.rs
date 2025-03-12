// Answer 0

#[test]
fn test_key_mut_with_single_entry() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;
        
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: vec![Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() }] };
    let mut occupied_entry = RawOccupiedEntryMut { entries: &mut entries, index: hash_table::OccupiedEntry::new(0), hash_builder: PhantomData };
    let key_mut = occupied_entry.key_mut();
}

#[test]
fn test_key_mut_with_multiple_entries() {
    struct TestEntries {
        entries: Vec<Bucket<String, i32>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<String, i32>;
        
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: vec![
        Bucket { hash: HashValue::default(), key: "first".to_string(), value: 2 },
        Bucket { hash: HashValue::default(), key: "second".to_string(), value: 3 },
    ]};
    let mut occupied_entry = RawOccupiedEntryMut { entries: &mut entries, index: hash_table::OccupiedEntry::new(1), hash_builder: PhantomData };
    let key_mut = occupied_entry.key_mut();
}

#[test]
fn test_key_mut_with_custom_struct() {
    #[derive(Clone)]
    struct CustomKey {
        id: i32,
        name: String,
    }

    struct TestEntries {
        entries: Vec<Bucket<CustomKey, bool>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<CustomKey, bool>;
        
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: vec![
        Bucket { hash: HashValue::default(), key: CustomKey { id: 1, name: "Alice".to_string() }, value: true },
    ]};
    let mut occupied_entry = RawOccupiedEntryMut { entries: &mut entries, index: hash_table::OccupiedEntry::new(0), hash_builder: PhantomData };
    let key_mut = occupied_entry.key_mut();
}

#[test]
#[should_panic]
fn test_key_mut_with_empty_entries() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;
        
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: vec![] };
    // Attempting to access key_mut on an empty Entries struct
    let mut occupied_entry = RawOccupiedEntryMut { entries: &mut entries, index: hash_table::OccupiedEntry::new(0), hash_builder: PhantomData };
    let key_mut = occupied_entry.key_mut();
}

