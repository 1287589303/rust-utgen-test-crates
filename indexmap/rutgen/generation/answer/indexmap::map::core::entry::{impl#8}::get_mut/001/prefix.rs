// Answer 0

#[test]
fn test_get_mut_valid_entry() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut {
        indices: &mut Indices::new(),
        entries: &mut Entries { data: entries },
    };
    let mut map = IndexMapCore { entries: ref_mut };
    let index = 1;

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let value_mut = indexed_entry.get_mut();
}

#[test]
fn test_get_mut_first_entry() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 30 },
    ];
    let mut ref_mut = RefMut {
        indices: &mut Indices::new(),
        entries: &mut Entries { data: entries },
    };
    let mut map = IndexMapCore { entries: ref_mut };
    let index = 0;

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let value_mut = indexed_entry.get_mut();
}

#[test]
fn test_get_mut_empty_entries_should_panic() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![];
    let mut ref_mut = RefMut {
        indices: &mut Indices::new(),
        entries: &mut Entries { data: entries },
    };
    let mut map = IndexMapCore { entries: ref_mut };
    let index = 0;

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let value_mut = indexed_entry.get_mut();
}

#[test]
fn test_get_mut_out_of_bounds_index_should_panic() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue(1), key: 3, value: 40 },
    ];
    let mut ref_mut = RefMut {
        indices: &mut Indices::new(),
        entries: &mut Entries { data: entries },
    };
    let mut map = IndexMapCore { entries: ref_mut };
    let index = 1; // Out of bounds

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let value_mut = indexed_entry.get_mut();
}

