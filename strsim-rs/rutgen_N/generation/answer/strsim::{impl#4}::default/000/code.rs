// Answer 0

#[test]
fn test_default_initialization() {
    struct TestStruct {
        used: i32,
        fill: i32,
        mask: i32,
        map: Option<i32>,
    }

    impl TestStruct {
        fn default() -> Self {
            Self {
                used: 0,
                fill: 0,
                mask: -1,
                map: None,
            }
        }
    }

    let instance = TestStruct::default();
    assert_eq!(instance.used, 0);
    assert_eq!(instance.fill, 0);
    assert_eq!(instance.mask, -1);
    assert_eq!(instance.map, None);
}

