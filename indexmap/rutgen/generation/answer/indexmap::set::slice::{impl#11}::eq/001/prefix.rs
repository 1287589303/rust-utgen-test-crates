// Answer 0

#[test]
fn test_eq_empty_arrays() {
    let self_array: [i32; 0] = [];
    let other_slice = Slice { entries: [] };
    let result = self_array.eq(&other_slice);
}

#[test]
fn test_eq_matching_arrays() {
    let self_array = [1, 2, 3];
    let other_slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
            Bucket { hash: 0, key: 2, value: "b" },
            Bucket { hash: 0, key: 3, value: "c" },
        ],
    };
    let result = self_array.eq(&other_slice);
}

#[test]
fn test_eq_non_matching_lengths() {
    let self_array = [1, 2, 3];
    let other_slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
            Bucket { hash: 0, key: 2, value: "b" },
        ],
    };
    let result = self_array.eq(&other_slice);
}

#[test]
fn test_eq_non_matching_values() {
    let self_array = [1, 2, 3];
    let other_slice = Slice {
        entries: [
            Bucket { hash: 0, key: 4, value: "d" },
            Bucket { hash: 0, key: 5, value: "e" },
            Bucket { hash: 0, key: 6, value: "f" },
        ],
    };
    let result = self_array.eq(&other_slice);
}

#[test]
fn test_eq_empty_self_array() {
    let self_array: [i32; 0] = [];
    let other_slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
        ],
    };
    let result = self_array.eq(&other_slice);
}

#[test]
fn test_eq_self_array_with_no_matching_keys() {
    let self_array = [5, 6, 7];
    let other_slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
            Bucket { hash: 0, key: 2, value: "b" },
            Bucket { hash: 0, key: 3, value: "c" },
        ],
    };
    let result = self_array.eq(&other_slice);
}

