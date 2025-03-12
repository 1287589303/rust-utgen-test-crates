// Answer 0

#[test]
fn test_fmt_with_r_prefix() {
    struct IdentFragment {
        value: String,
    }

    impl IdentFragment {
        fn to_string(&self) -> String {
            self.value.clone()
        }
    }

    use std::fmt;

    impl fmt::Display for IdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let id = self.to_string();
            if let Some(id) = id.strip_prefix("r#") {
                fmt::Display::fmt(id, f)
            } else {
                fmt::Display::fmt(&id[..], f)
            }
        }
    }

    let ident = IdentFragment { value: String::from("r#my_identifier") };
    let mut output = String::new();
    let result = write!(&mut output, "{}", ident);
    assert!(result.is_ok());
    assert_eq!(output, "my_identifier");
}

#[test]
fn test_fmt_with_empty_r_prefix() {
    struct IdentFragment {
        value: String,
    }

    impl IdentFragment {
        fn to_string(&self) -> String {
            self.value.clone()
        }
    }

    use std::fmt;

    impl fmt::Display for IdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let id = self.to_string();
            if let Some(id) = id.strip_prefix("r#") {
                fmt::Display::fmt(id, f)
            } else {
                fmt::Display::fmt(&id[..], f)
            }
        }
    }

    let ident = IdentFragment { value: String::from("r#") };
    let mut output = String::new();
    let result = write!(&mut output, "{}", ident);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

