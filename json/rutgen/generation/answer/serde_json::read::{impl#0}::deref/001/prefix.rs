// Answer 0

#[test]
fn test_deref_copied_str() {
    struct TestStruct;
    impl Deref for TestStruct {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            "test"
        }
    }

    let value = TestStruct;
    let reference = Reference::Copied(&value);
    let _result: &str = reference.deref();
}

#[test]
fn test_deref_copied_vec() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl Deref for TestStruct {
        type Target = Vec<i32>;
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    let value = TestStruct { data: vec![1, 2, 3] };
    let reference = Reference::Copied(&value);
    let _result: &Vec<i32> = reference.deref();
}

#[test]
fn test_deref_copied_integer() {
    struct TestStruct(i32);

    impl Deref for TestStruct {
        type Target = i32;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let value = TestStruct(42);
    let reference = Reference::Copied(&value);
    let _result: &i32 = reference.deref();
}

