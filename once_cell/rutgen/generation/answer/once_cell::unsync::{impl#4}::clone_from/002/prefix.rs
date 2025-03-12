// Answer 0

#[test]
fn test_clone_from_self_none_source_some() {
    let source = OnceCell::with_value(42);
    let mut self_none: OnceCell<i32> = OnceCell::new();
    self_none.clone_from(&source);
}

#[test]
fn test_clone_from_self_some_source_none() {
    let mut self_some = OnceCell::with_value(42);
    let source: OnceCell<i32> = OnceCell::new();
    self_some.clone_from(&source);
}

#[test]
fn test_clone_from_self_none_source_none() {
    let source: OnceCell<i32> = OnceCell::new();
    let mut self_none: OnceCell<i32> = OnceCell::new();
    self_none.clone_from(&source);
}

