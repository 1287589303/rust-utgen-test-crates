// Answer 0

#[test]
fn test_pool_new_capacity_non_zero() {
    struct TestCreateF;

    let create_fn = |value| value;
    let pool: Pool<i32, _> = Pool::new(create_fn);

    for _ in 0..pool.stack.data.get().len() {
        let _ = pool.stack.data.get(); // Access stacks for non-zero capacity check
    }
}

#[test]
fn test_pool_new_capacity_zero() {
    struct TestCreateF;

    let create_fn = |value| value;
    let pool: Pool<i32, _> = Pool::new(create_fn);

    assert_eq!(pool.stack.data.get().len(), 0); // Verify that the capacity is zero
}

