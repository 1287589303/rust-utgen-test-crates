// Answer 0

#[test]
fn test_split_at_empty_slice() {
    let slice: &Slice<i32, i32> = Slice::new();
    let result = slice.split_at(0);
}

#[test]
fn test_split_at_single_element_slice() {
    let bucket = Bucket { hash: 0, key: 1, value: 10 };
    let entries = [bucket];
    let slice: &Slice<i32, i32> = Slice::from_slice(&entries);
    let result = slice.split_at(1);
}

#[test]
#[should_panic]
fn test_split_at_out_of_bounds() {
    let bucket = Bucket { hash: 0, key: 1, value: 10 };
    let entries = [bucket];
    let slice: &Slice<i32, i32> = Slice::from_slice(&entries);
    let result = slice.split_at(2);
}

#[test]
fn test_split_at_first_element() {
    let bucket1 = Bucket { hash: 0, key: 1, value: 10 };
    let bucket2 = Bucket { hash: 0, key: 2, value: 20 };
    let entries = [bucket1, bucket2];
    let slice: &Slice<i32, i32> = Slice::from_slice(&entries);
    let result = slice.split_at(1);
}

#[test]
fn test_split_at_mid_element() {
    let bucket1 = Bucket { hash: 0, key: 1, value: 10 };
    let bucket2 = Bucket { hash: 0, key: 2, value: 20 };
    let bucket3 = Bucket { hash: 0, key: 3, value: 30 };
    let entries = [bucket1, bucket2, bucket3];
    let slice: &Slice<i32, i32> = Slice::from_slice(&entries);
    let result = slice.split_at(2);
}

