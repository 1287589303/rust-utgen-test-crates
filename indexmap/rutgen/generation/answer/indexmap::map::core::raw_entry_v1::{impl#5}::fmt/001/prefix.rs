// Answer 0

#[test]
fn test_raw_entry_mut_vacant_fmt() {
    struct TestHashBuilder;

    let map = RefMut::new(IndexMap::new());
    let hash_builder = &TestHashBuilder;

    let vacant_entry = RawVacantEntryMut {
        map,
        hash_builder,
    };

    let raw_entry_mut = RawEntryMut::Vacant(vacant_entry);

    let mut formatter = fmt::Formatter::new();
    let _ = raw_entry_mut.fmt(&mut formatter);
}

#[test]
fn test_raw_entry_mut_occupied_fmt() {
    struct TestHashBuilder;

    let mut entries = Entries::new();
    let hash_builder = &TestHashBuilder;

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(),
        hash_builder: PhantomData,
    };

    let raw_entry_mut = RawEntryMut::Occupied(occupied_entry);

    let mut formatter = fmt::Formatter::new();
    let _ = raw_entry_mut.fmt(&mut formatter);
}

