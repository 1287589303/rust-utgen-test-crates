// Answer 0

#[test]
fn test_index_empty() {
    let mut indices = vec![];
    let entries = Entries::<i32, i32>::new();
    let map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: map, hash_builder: &Default::default() };
    let _ = entry.index();
}

#[test]
fn test_index_single_element() {
    let mut indices = vec![0];
    let entries = Entries::<i32, i32>::new();
    let map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: map, hash_builder: &Default::default() };
    let _ = entry.index();
}

#[test]
fn test_index_multiple_elements() {
    let mut indices = vec![0, 1, 2, 3, 4];
    let entries = Entries::<i32, i32>::new();
    let map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: map, hash_builder: &Default::default() };
    let _ = entry.index();
}

#[test]
fn test_index_boundary_max() {
    let mut indices = (0..1000).collect::<Vec<_>>();
    let entries = Entries::<i32, i32>::new();
    let map = RefMut { indices: &mut indices, entries: &mut entries };
    let entry = RawVacantEntryMut { map: map, hash_builder: &Default::default() };
    let _ = entry.index();
}

