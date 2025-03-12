// Answer 0

#[test]
fn test_increment_indices_case_1() {
    let mut indices = hash_table::HashTable::with_capacity(4);
    indices.insert(0, 0);
    indices.insert(1, 1);
    indices.insert(2, 2);
    indices.insert(3, 3); // Capacity of indices is 4

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
        Bucket { hash: HashValue(3), key: 4, value: 40 },
    ]; // entries.len() == 4

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(1, 3); // boundary: shifted_entries.len() == 2 (capacity / 2)
}

#[test]
fn test_increment_indices_case_2() {
    let mut indices = hash_table::HashTable::with_capacity(6);
    indices.insert(0, 0);
    indices.insert(1, 1);
    indices.insert(2, 2); // Capacity of indices is 6

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
    ]; // entries.len() == 3

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 3); // boundary: shifted_entries.len() == 3
}

#[test]
fn test_increment_indices_case_3() {
    let mut indices = hash_table::HashTable::with_capacity(2);
    indices.insert(3, 0); // all indices < start (start=0)
    indices.insert(4, 1);

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ]; // entries.len() == 2

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 2); // boundary: shifted_entries.len() == 2
}

#[test]
fn test_increment_indices_case_4() {
    let mut indices = hash_table::HashTable::with_capacity(4);
    indices.insert(4, 0); // all indices >= end (end=3)
    indices.insert(5, 1);

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
        Bucket { hash: HashValue(3), key: 4, value: 40 },
    ]; // entries.len() == 4

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(2, 4); // boundary: shifted_entries.len() == 2
}

