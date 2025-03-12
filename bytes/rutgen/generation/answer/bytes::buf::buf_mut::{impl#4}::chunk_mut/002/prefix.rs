// Answer 0

#[test]
fn test_chunk_mut_nonempty_vec() {
    let mut vec = Vec::with_capacity(64);
    vec.push(1);
    let result: &mut UninitSlice = vec.chunk_mut();
}

#[test]
fn test_chunk_mut_partially_filled_vec() {
    let mut vec = Vec::with_capacity(128);
    vec.push(1);
    vec.push(2);
    let result: &mut UninitSlice = vec.chunk_mut();
}

#[test]
fn test_chunk_mut_almost_full_vec() {
    let mut vec = Vec::with_capacity(256);
    for _ in 0..255 {
        vec.push(1);
    }
    let result: &mut UninitSlice = vec.chunk_mut();
}

#[test]
fn test_chunk_mut_empty_vec_initialization() {
    let mut vec: Vec<u8> = Vec::with_capacity(32);
    // capacity is greater than length (which is 0)
    let result: &mut UninitSlice = vec.chunk_mut();
}

