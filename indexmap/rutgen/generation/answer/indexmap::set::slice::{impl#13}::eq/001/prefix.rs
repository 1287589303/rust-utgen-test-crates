// Answer 0

#[test]
fn test_eq_with_empty_arrays() {
    let arr: [i32; 0] = [];
    let slice = Slice { entries: [] };
    arr.eq(&slice);
}

#[test]
fn test_eq_with_same_element() {
    let arr: [i32; 1] = [1];
    let slice = Slice { entries: [Bucket::new(1)] };
    arr.eq(&slice);
}

#[test]
fn test_eq_with_different_elements() {
    let arr: [i32; 1] = [1];
    let slice = Slice { entries: [Bucket::new(2)] };
    arr.eq(&slice);
}

#[test]
fn test_eq_with_empty_and_non_empty() {
    let arr: [i32; 0] = [];
    let slice = Slice { entries: [Bucket::new(1)] };
    arr.eq(&slice);
}

#[test]
fn test_eq_with_identical_elements() {
    let arr: [i32; 3] = [1, 2, 3];
    let slice = Slice { entries: [Bucket::new(1), Bucket::new(2), Bucket::new(3)] };
    arr.eq(&slice);
}

#[test]
fn test_eq_with_different_lengths() {
    let arr: [i32; 3] = [1, 2, 3];
    let slice = Slice { entries: [Bucket::new(1), Bucket::new(2)] };
    arr.eq(&slice);
}

#[test]
fn test_eq_with_one_different_element() {
    let arr: [i32; 3] = [1, 2, 3];
    let slice = Slice { entries: [Bucket::new(1), Bucket::new(4), Bucket::new(3)] };
    arr.eq(&slice);
}

#[test]
fn test_eq_with_max_capacity_array() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let entries = [
        Bucket::new(1),
        Bucket::new(2),
        Bucket::new(3),
        Bucket::new(4),
        Bucket::new(5),
    ];
    let slice = Slice { entries };
    arr.eq(&slice);
}

