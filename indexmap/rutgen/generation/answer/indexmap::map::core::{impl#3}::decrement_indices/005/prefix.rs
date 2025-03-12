// Answer 0

#[test]
fn test_decrement_indices_half_capacity() {
    let mut indices = indexmap::Indices::with_capacity(8);
    indices.push(1);
    indices.push(2);
    indices.push(3);
    indices.push(4);
    
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: "a", value: 10 },
        Bucket { hash: HashValue(2), key: "b", value: 20 },
        Bucket { hash: HashValue(3), key: "c", value: 30 },
        Bucket { hash: HashValue(4), key: "d", value: 40 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 4);
}

#[test]
fn test_decrement_indices_equal_capacity() {
    let mut indices = indexmap::Indices::with_capacity(4);
    indices.push(1);
    indices.push(2);
    
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: "x", value: 100 },
        Bucket { hash: HashValue(2), key: "y", value: 200 },
        Bucket { hash: HashValue(3), key: "z", value: 300 },
        Bucket { hash: HashValue(4), key: "w", value: 400 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 4);
}

#[test]
fn test_decrement_indices_zero_range() {
    let mut indices = indexmap::Indices::with_capacity(8);
    
    let mut entries: Vec<Bucket<_, _>> = Vec::new();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 0);
}

