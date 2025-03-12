// Answer 0

#[test]
fn test_into_iter_empty_vec() {
    let builder: RcVecBuilder<i32> = RcVecBuilder { inner: Vec::new() };
    let _iter = builder.into_iter();
}

#[test]
fn test_into_iter_single_element_vec() {
    let builder: RcVecBuilder<i32> = RcVecBuilder { inner: vec![42] };
    let _iter = builder.into_iter();
}

#[test]
fn test_into_iter_multiple_elements_vec() {
    let builder: RcVecBuilder<i32> = RcVecBuilder { inner: vec![1, 2, 3, 4, 5] };
    let _iter = builder.into_iter();
}

#[test]
fn test_into_iter_large_vec() {
    let large_vec = (0..1000).collect::<Vec<i32>>();
    let builder: RcVecBuilder<i32> = RcVecBuilder { inner: large_vec };
    let _iter = builder.into_iter();
}

