// Answer 0

#[test]
fn test_split_at_empty() {
    let slice: Box<Slice<u32>> = Box::new(Slice::new());
    let index = 0;
    let (first, second) = slice.split_at(index);
    let first_slice = first.len();
    let second_slice = second.len();
}

#[test]
fn test_split_at_single_element() {
    let slice: Box<Slice<u32>> = Box::new(Slice::from_slice(&[Bucket { hash: 0, key: 1, value: 10 }]));
    let index = 1;
    let (first, second) = slice.split_at(index);
    let first_slice = first.len();
    let second_slice = second.len();
}

#[test]
fn test_split_at_multiple_elements() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
    ];
    let slice: Box<Slice<u32>> = Box::new(Slice::from_slice(&entries));
    let index = 2;
    let (first, second) = slice.split_at(index);
    let first_slice = first.len();
    let second_slice = second.len();
}

#[test]
#[should_panic]
fn test_split_at_out_of_bounds() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
    ];
    let slice: Box<Slice<u32>> = Box::new(Slice::from_slice(&entries));
    let index = 2; // Out of bounds
    let _ = slice.split_at(index);
}

