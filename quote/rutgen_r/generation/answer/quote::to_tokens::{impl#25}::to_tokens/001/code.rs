// Answer 0

#[test]
fn test_to_tokens_with_valid_string() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::c_string(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_instance = TestStruct { value: "Hello, world!" };
    test_instance.to_tokens(&mut tokens);
    assert!(!tokens.is_empty());
}

#[test]
fn test_to_tokens_with_empty_string() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::c_string(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_instance = TestStruct { value: "" };
    test_instance.to_tokens(&mut tokens);
    assert!(!tokens.is_empty());
}

#[should_panic]
fn test_to_tokens_with_null_string() {
    struct TestStruct {
        value: *const i8,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            if self.value.is_null() {
                panic!("Null value encountered");
            }
            tokens.append(Literal::c_string(unsafe { std::ffi::CStr::from_ptr(self.value).to_str().unwrap() }));
        }
    }

    let mut tokens = TokenStream::new();
    let test_instance = TestStruct { value: std::ptr::null() };
    test_instance.to_tokens(&mut tokens);
}

