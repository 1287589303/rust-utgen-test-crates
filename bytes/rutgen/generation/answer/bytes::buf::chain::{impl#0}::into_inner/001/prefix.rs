// Answer 0

#[test]
fn test_into_inner_byte_slices() {
    let chain = Chain::new(&b"test"[..], &b"chain"[..]);
    let (first, last) = chain.into_inner();
}

#[test]
fn test_into_inner_vecs() {
    let chain = Chain::new(vec![1, 2, 3], vec![4, 5, 6]);
    let (first, last) = chain.into_inner();
}

#[test]
fn test_into_inner_strings() {
    let chain = Chain::new(String::from("hello"), String::from("world"));
    let (first, last) = chain.into_inner();
}

#[test]
fn test_into_inner_option() {
    let chain = Chain::new(Some(42), None);
    let (first, last) = chain.into_inner();
}

#[test]
fn test_into_inner_empty_slices() {
    let chain = Chain::new(&b""[..], &b""[..]);
    let (first, last) = chain.into_inner();
}

#[test]
fn test_into_inner_empty_vecs() {
    let chain = Chain::new(vec![], vec![]);
    let (first, last) = chain.into_inner();
}

#[test]
fn test_into_inner_large_vecs() {
    let large_vec_a = (0..1000).collect::<Vec<_>>();
    let large_vec_b = (1000..2000).collect::<Vec<_>>();
    let chain = Chain::new(large_vec_a, large_vec_b);
    let (first, last) = chain.into_inner();
}

