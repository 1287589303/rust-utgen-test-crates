// Answer 0

#[test]
fn test_shift_remove_finish_valid_index_zero() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue::new(1), key: 0, value: "zero" },
        Bucket { hash: HashValue::new(2), key: 1, value: "one" }
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.shift_remove_finish(0);
}

#[test]
fn test_shift_remove_finish_valid_index_one() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue::new(1), key: 0, value: "zero" },
        Bucket { hash: HashValue::new(2), key: 1, value: "one" }
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.shift_remove_finish(1);
}

#[test]
#[should_panic]
fn test_shift_remove_finish_out_of_bounds_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue::new(1), key: 0, value: "zero" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.shift_remove_finish(2);
}

