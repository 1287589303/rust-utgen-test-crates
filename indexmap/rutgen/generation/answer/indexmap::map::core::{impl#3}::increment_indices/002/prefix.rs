// Answer 0

#[test]
fn test_increment_indices_case_1() {
    let mut indices = hash_table::HashTable::<usize>::new();
    indices.insert(0);
    indices.insert(1);
    let entries = vec![Bucket { hash: HashValue(0), key: 100, value: "a" }, 
                       Bucket { hash: HashValue(1), key: 200, value: "b" },
                       Bucket { hash: HashValue(2), key: 300, value: "c" }];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(1, 3);
}

#[test]
fn test_increment_indices_case_2() {
    let mut indices = hash_table::HashTable::<usize>::new();
    indices.insert(1);
    indices.insert(2);
    indices.insert(3);
    
    let entries = vec![Bucket { hash: HashValue(3), key: 400, value: "d" }, 
                       Bucket { hash: HashValue(4), key: 500, value: "e" },
                       Bucket { hash: HashValue(5), key: 600, value: "f" },
                       Bucket { hash: HashValue(6), key: 700, value: "g" }];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(2, 4);
}

#[test]
fn test_increment_indices_case_3() {
    let mut indices = hash_table::HashTable::<usize>::new();
    indices.insert(2);
    indices.insert(3);
    indices.insert(4);
    indices.insert(5);
    
    let entries = vec![Bucket { hash: HashValue(7), key: 800, value: "h" },
                       Bucket { hash: HashValue(8), key: 900, value: "i" },
                       Bucket { hash: HashValue(9), key: 1000, value: "j" },
                       Bucket { hash: HashValue(10), key: 1100, value: "k" },
                       Bucket { hash: HashValue(11), key: 1200, value: "l" }];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(3, 5);
}

