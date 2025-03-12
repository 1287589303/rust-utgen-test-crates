// Answer 0

#[test]
fn test_chunk_non_empty_slice() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let result = data.chunk();
}

#[test]
fn test_chunk_another_non_empty_slice() {
    let data: &[u8] = &[10, 20, 30, 40, 50];
    let result = data.chunk();
}

