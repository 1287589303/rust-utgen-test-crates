// Answer 0

#[test]
fn test_into_ref_mut_with_start_index() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
    ];
    let index = hash_table::OccupiedEntry::from_index(0);
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };
    let _ref_mut = raw_entry.into_ref_mut();
}

#[test]
fn test_into_ref_mut_with_end_index() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
    ];
    let index = hash_table::OccupiedEntry::from_index(entries.len() - 1);
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };
    let _ref_mut = raw_entry.into_ref_mut();
}

#[test]
fn test_into_ref_mut_with_middle_index() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
        Bucket { hash: HashValue::default(), key: 3, value: 30 },
    ];
    let index = hash_table::OccupiedEntry::from_index(1);
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };
    let _ref_mut = raw_entry.into_ref_mut();
}

#[test]
fn test_into_ref_mut_with_single_entry() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
    ];
    let index = hash_table::OccupiedEntry::from_index(0);
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };
    let _ref_mut = raw_entry.into_ref_mut();
}

