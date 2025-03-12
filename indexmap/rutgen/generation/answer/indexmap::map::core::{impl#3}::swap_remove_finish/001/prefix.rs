// Answer 0

#[test]
fn test_swap_remove_finish_non_last_element() {
    let mut indices = Indices::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 100 },
        Bucket { hash: HashValue(1), key: 2, value: 200 },
        Bucket { hash: HashValue(2), key: 3, value: 300 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 1;
    let result = ref_mut.swap_remove_finish(index);
}

#[test]
fn test_swap_remove_finish_last_element() {
    let mut indices = Indices::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 100 },
        Bucket { hash: HashValue(1), key: 2, value: 200 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 0; // ensure this is the last element (1 will be removed)
    let result = ref_mut.swap_remove_finish(index);
}

#[test]
fn test_swap_remove_finish_with_multiple_elements() {
    let mut indices = Indices::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 100 },
        Bucket { hash: HashValue(1), key: 2, value: 200 },
        Bucket { hash: HashValue(2), key: 3, value: 300 },
        Bucket { hash: HashValue(3), key: 4, value: 400 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 2; // choose an element in the middle
    let result = ref_mut.swap_remove_finish(index);
}

#[test]
fn test_swap_remove_finish_first_element() {
    let mut indices = Indices::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 100 },
        Bucket { hash: HashValue(1), key: 2, value: 200 },
        Bucket { hash: HashValue(2), key: 3, value: 300 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 0; // first element
    let result = ref_mut.swap_remove_finish(index);
}

