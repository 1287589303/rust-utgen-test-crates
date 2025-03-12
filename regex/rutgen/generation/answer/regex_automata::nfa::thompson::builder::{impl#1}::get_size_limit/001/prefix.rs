// Answer 0

#[test]
fn test_get_size_limit_none() {
    let builder = Builder::default();
    let limit = builder.get_size_limit();
}

#[test]
fn test_get_size_limit_some_zero() {
    let mut builder = Builder::default();
    builder.set_size_limit(Some(0)).unwrap();
    let limit = builder.get_size_limit();
}

#[test]
fn test_get_size_limit_some_one() {
    let mut builder = Builder::default();
    builder.set_size_limit(Some(1)).unwrap();
    let limit = builder.get_size_limit();
}

#[test]
fn test_get_size_limit_some_ustex_max() {
    let mut builder = Builder::default();
    builder.set_size_limit(Some(usize::MAX)).unwrap();
    let limit = builder.get_size_limit();
}

