// Answer 0

#[test]
fn test_get_or_try_init_value_already_set_fail_set() {
    struct TestOnceCell {
        once_cell: OnceCell<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell {
                once_cell: OnceCell::new(),
            }
        }
        
        fn initialize(&mut self) {
            let _ = self.once_cell.set(92);
        }
    }

    let mut test_cell = TestOnceCell::new();
    test_cell.initialize();
    
    let result = test_cell.once_cell.get_or_try_init(|| Ok(100));
    let _ = result.unwrap();
    
    let set_result = test_cell.once_cell.set(200);
    let _ = set_result.unwrap_err();
}

