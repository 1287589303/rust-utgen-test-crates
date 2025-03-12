// Answer 0

#[test]
fn test_partial_shuffle_equal_length() {
    let mut data = [1, 2, 3, 4, 5];
    let amount = data.len();
    let mut rng = rand::thread_rng();
    let result = data.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_single_element() {
    let mut data = [42];
    let amount = data.len();
    let mut rng = rand::thread_rng();
    let result = data.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_empty_slice() {
    let mut data: Vec<i32> = Vec::new();
    let amount = 0;
    let mut rng = rand::thread_rng();
    let result = data.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_large_slice() {
    let mut data: Vec<i32> = (1..100).collect();
    let amount = data.len();
    let mut rng = rand::thread_rng();
    let result = data.partial_shuffle(&mut rng, amount);
} 

#[test]
fn test_partial_shuffle_single_shuffle() {
    let mut data = [1, 2, 3];
    let amount = 1;
    let mut rng = rand::thread_rng();
    let result = data.partial_shuffle(&mut rng, amount);
}

