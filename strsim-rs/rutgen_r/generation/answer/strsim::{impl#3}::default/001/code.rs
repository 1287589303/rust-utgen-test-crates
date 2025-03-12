// Answer 0

#[test]
fn test_default() {
    struct TestStruct {
        val: i32,
    }
    
    impl TestStruct {
        fn default() -> Self {
            Self { val: -1 }
        }
    }
    
    let instance = TestStruct::default();
    assert_eq!(instance.val, -1);
}

