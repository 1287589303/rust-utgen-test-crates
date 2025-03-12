// Answer 0

#[test]
fn test_swap_indices_valid() {
    let mut index_map = IndexMapCore::<usize, usize>::new();

    // Adding some entries for testing
    index_map.entries.push(Bucket { key: 1, value: 10 });
    index_map.entries.push(Bucket { key: 2, value: 20 });
    index_map.entries.push(Bucket { key: 3, value: 30 });

    index_map.swap_indices(0, 1);
}

#[test]
fn test_swap_indices_boundary_case_low() {
    let mut index_map = IndexMapCore::<usize, usize>::new();

    index_map.entries.push(Bucket { key: 1, value: 10 });
    index_map.entries.push(Bucket { key: 2, value: 20 });

    index_map.swap_indices(0, 1);
}

#[test]
fn test_swap_indices_boundary_case_high() {
    let mut index_map = IndexMapCore::<usize, usize>::new();

    index_map.entries.push(Bucket { key: 1, value: 10 });
    index_map.entries.push(Bucket { key: 2, value: 20 });
    index_map.entries.push(Bucket { key: 3, value: 30 });

    index_map.swap_indices(1, 2);
}

#[test]
#[should_panic]
fn test_swap_indices_invalid_a() {
    let mut index_map = IndexMapCore::<usize, usize>::new();

    index_map.entries.push(Bucket { key: 1, value: 10 });
    index_map.entries.push(Bucket { key: 2, value: 20 });

    index_map.swap_indices(2, 1); // Invalid: `a` is out of bounds
}

#[test]
#[should_panic]
fn test_swap_indices_invalid_b() {
    let mut index_map = IndexMapCore::<usize, usize>::new();

    index_map.entries.push(Bucket { key: 1, value: 10 });
    index_map.entries.push(Bucket { key: 2, value: 20 });

    index_map.swap_indices(1, 2); // Invalid: `b` is out of bounds
}

#[test]
#[should_panic]
fn test_swap_indices_equal() {
    let mut index_map = IndexMapCore::<usize, usize>::new();

    index_map.entries.push(Bucket { key: 1, value: 10 });
    index_map.entries.push(Bucket { key: 2, value: 20 });

    index_map.swap_indices(1, 1); // Invalid: `a` and `b` are equal
}

