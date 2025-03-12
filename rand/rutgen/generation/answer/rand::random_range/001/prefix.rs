// Answer 0

#[test]
fn test_random_range_f32() {
    let _value: f32 = rand::random_range(0.0..=1.0);
    let _value_empty: f32 = rand::random_range(1.0..=0.0); // empty range
    let _value_large: f32 = rand::random_range(0.0..=1e9); // large range
}

#[test]
fn test_random_range_i32() {
    let _value: i32 = rand::random_range(0..=10);
    let _value_empty: i32 = rand::random_range(10..=0); // empty range
    let _value_large: i32 = rand::random_range(-1000..=1000); // large range
    let _value_single: i32 = rand::random_range(5..=5); // single value range
}

#[test]
fn test_random_range_usize() {
    let _value: usize = rand::random_range(0..=100);
    let _value_empty: usize = rand::random_range(100..=0); // empty range
    let _value_large: usize = rand::random_range(usize::MIN..=usize::MAX); // large range
} 

#[test]
fn test_random_range_string_indices() {
    let words: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    let _index: usize = rand::random_range(..words.len()); // valid range
    let _index_empty: usize = rand::random_range(words.len()..words.len()); // empty range
}

