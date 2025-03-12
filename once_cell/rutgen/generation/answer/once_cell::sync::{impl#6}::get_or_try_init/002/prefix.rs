// Answer 0

#[test]
fn test_get_or_try_init_success() {
    struct TestCell {
        once_cell: OnceCell<i32>,
    }

    let cell = TestCell {
        once_cell: OnceCell::new(),
    };

    // First call returns None
    assert!(cell.once_cell.get().is_none());

    // Initialize cell successfully
    let result = cell.once_cell.get_or_try_init(|| Ok(42));
    // Check that we received Ok with the reference
    let expected = Ok(&42);
    let actual = result;
}

#[test]
fn test_get_or_try_init_multiple_calls() {
    struct TestCell {
        once_cell: OnceCell<i32>,
    }

    let cell = TestCell {
        once_cell: OnceCell::new(),
    };

    // First call returns None
    assert!(cell.once_cell.get().is_none());

    // Initialize cell successfully
    let _ = cell.once_cell.get_or_try_init(|| Ok(99));
    
    // Second call should retrieve the previously initialized value
    let second_result = cell.once_cell.get_or_try_init(|| Ok(200)); // Should not re-initialize
    let expected_second = Ok(&99);
    let actual_second = second_result;
}

