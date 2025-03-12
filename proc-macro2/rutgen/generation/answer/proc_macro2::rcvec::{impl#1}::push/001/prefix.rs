// Answer 0

#[test]
fn test_push_with_zero_initial_capacity() {
    let mut builder = RcVecBuilder::<i32>::with_capacity(0);
    builder.push(1);
}

#[test]
fn test_push_with_positive_initial_capacity() {
    let mut builder = RcVecBuilder::<i32>::with_capacity(10);
    builder.push(2);
}

#[test]
fn test_push_multiple_elements() {
    let mut builder = RcVecBuilder::<i32>::new();
    builder.push(10);
    builder.push(20);
    builder.push(30);
}

#[test]
fn test_push_boundary_elements() {
    let mut builder = RcVecBuilder::<usize>::with_capacity(100);
    for i in 0..100 {
        builder.push(i);
    }
}

#[test]
fn test_push_null_elements() {
    let mut builder = RcVecBuilder::<Option<String>>::new();
    builder.push(None);
}

#[test]
fn test_push_memory_safety() {
    let mut builder = RcVecBuilder::<String>::new();
    builder.push("Hello".to_string());
    builder.push("World".to_string());
}

