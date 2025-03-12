// Answer 0

#[test]
fn test_partial_shuffle_basic() {
    let mut rng = rand::thread_rng();
    let mut slice = [1, 2, 3, 4, 5];
    let amount = 0;
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_full() {
    let mut rng = rand::thread_rng();
    let mut slice = [1, 2, 3, 4, 5];
    let amount = slice.len();
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_non_empty() {
    let mut rng = rand::thread_rng();
    let mut slice = [1, 2, 3, 4, 5];
    let amount = 3;
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_boundary() {
    let mut rng = rand::thread_rng();
    let mut slice = [1, 2];
    let amount = 1;
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_larger_slice() {
    let mut rng = rand::thread_rng();
    let mut slice: Vec<u32> = (1..1000).collect();
    let amount = 500;
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
#[should_panic]
fn test_partial_shuffle_exceeding_amount() {
    let mut rng = rand::thread_rng();
    let mut slice = [1, 2, 3];
    let amount = 5; // intentionally exceeding the length of the slice
    let result = slice.partial_shuffle(&mut rng, amount);
}

