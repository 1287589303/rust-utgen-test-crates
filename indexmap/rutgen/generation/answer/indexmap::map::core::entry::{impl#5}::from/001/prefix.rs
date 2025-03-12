// Answer 0

#[test]
fn test_from_with_valid_indexed_entry() {
    struct TestEntries {
        entries: Vec<Bucket<u32, u32>>,
        indices: hashbrown::HashMap<u64, usize>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, u32>;

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

    let bucket1 = Bucket { hash: HashValue(1), key: 10, value: 100 };
    let bucket2 = Bucket { hash: HashValue(2), key: 20, value: 200 };
    let entries = vec![bucket1, bucket2];
    let mut indices = hashbrown::HashMap::new();
    indices.insert(1, 0);
    indices.insert(2, 1);
    
    let test_entries = TestEntries { entries, indices };

    let indexed_entry = IndexedEntry {
        map: RefMut { indices: &mut test_entries.indices, entries: &mut test_entries.entries }, 
        index: 1,
    };

    let _occupied_entry: OccupiedEntry<u32, u32> = occupied_entry.from(indexed_entry);
}

#[test]
fn test_from_with_empty_entries() {
    struct TestEntries {
        entries: Vec<Bucket<u32, u32>>,
        indices: hashbrown::HashMap<u64, usize>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, u32>;

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

    let entries: Vec<Bucket<u32, u32>> = Vec::new();
    let indices = hashbrown::HashMap::new();

    let test_entries = TestEntries { entries, indices };

    // This should panic due to empty entries
    let indexed_entry = IndexedEntry {
        map: RefMut { indices: &mut test_entries.indices, entries: &mut test_entries.entries }, 
        index: 0,
    };

    let _occupied_entry: OccupiedEntry<u32, u32> = occupied_entry.from(indexed_entry);
}

#[test]
fn test_from_with_invalid_index() {
    struct TestEntries {
        entries: Vec<Bucket<u32, u32>>,
        indices: hashbrown::HashMap<u64, usize>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, u32>;

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

    let bucket = Bucket { hash: HashValue(1), key: 10, value: 100 };
    let entries = vec![bucket];
    let mut indices = hashbrown::HashMap::new();
    indices.insert(1, 0);
    
    let test_entries = TestEntries { entries, indices };

    // Using an index that is out of bounds
    let indexed_entry = IndexedEntry {
        map: RefMut { indices: &mut test_entries.indices, entries: &mut test_entries.entries }, 
        index: 1,
    };

    // This should panic due to index being out of bounds
    let _occupied_entry: OccupiedEntry<u32, u32> = occupied_entry.from(indexed_entry);
}

