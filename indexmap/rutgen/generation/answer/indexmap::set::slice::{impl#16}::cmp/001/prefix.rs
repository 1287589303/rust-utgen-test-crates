// Answer 0

#[test]
fn test_cmp_empty_slices() {
    let slice1 = Slice::new();
    let slice2 = Slice::new();
    slice1.cmp(&slice2);
}

#[test]
fn test_cmp_single_element_slices_equal() {
    let slice1 = Box::new(Slice { entries: [Bucket { hash: 0, key: 1, value: "a" }] });
    let slice2 = Box::new(Slice { entries: [Bucket { hash: 0, key: 1, value: "a" }] });
    slice1.cmp(&slice2);
}

#[test]
fn test_cmp_single_element_slices_different() {
    let slice1 = Box::new(Slice { entries: [Bucket { hash: 0, key: 1, value: "a" }] });
    let slice2 = Box::new(Slice { entries: [Bucket { hash: 0, key: 2, value: "b" }] });
    slice1.cmp(&slice2);
}

#[test]
fn test_cmp_multiple_elements_slices_equal() {
    let slice1 = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]});
    let slice2 = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]});
    slice1.cmp(&slice2);
}

#[test]
fn test_cmp_multiple_elements_slices_different() {
    let slice1 = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]});
    let slice2 = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 1, key: 1, value: "a" },
    ]});
    slice1.cmp(&slice2);
}

#[test]
fn test_cmp_reverse_ordered_slices() {
    let slice1 = Box::new(Slice { entries: [
        Bucket { hash: 1, key: 2, value: "b" },
        Bucket { hash: 0, key: 1, value: "a" },
    ]});
    let slice2 = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]});
    slice1.cmp(&slice2);
}

