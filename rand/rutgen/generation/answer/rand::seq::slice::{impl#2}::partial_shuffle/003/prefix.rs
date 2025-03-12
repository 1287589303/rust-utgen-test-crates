// Answer 0

#[test]
fn test_partial_shuffle_length_greater_than_max() {
    let mut slice: Vec<u32> = (0..(u32::MAX as usize) + 1).collect();
    let amount = 0; 
    let rng = &mut rand::thread_rng();
    let result = slice.partial_shuffle(rng, amount);
}

#[test]
fn test_partial_shuffle_length_exactly_max() {
    let mut slice: Vec<u32> = (0..(u32::MAX as usize)).collect();
    let amount = 0; 
    let rng = &mut rand::thread_rng();
    let result = slice.partial_shuffle(rng, amount);
}

#[test]
fn test_partial_shuffle_length_max_increment() {
    let mut slice: Vec<u32> = (0..(u32::MAX as usize)).collect();
    let amount = (u32::MAX as usize) - 1; 
    let rng = &mut rand::thread_rng();
    let result = slice.partial_shuffle(rng, amount);
} 

#[test]
fn test_partial_shuffle_length_greater_max_with_amount() {
    let mut slice: Vec<u32> = (0..(u32::MAX as usize) + 2).collect();
    let amount = 1; 
    let rng = &mut rand::thread_rng();
    let result = slice.partial_shuffle(rng, amount);
} 

#[test]
fn test_partial_shuffle_empty_amount() {
    let mut slice: Vec<u32> = (0..(u32::MAX as usize)).collect();
    let rng = &mut rand::thread_rng();
    let result = slice.partial_shuffle(rng, (u32::MAX as usize));
}

