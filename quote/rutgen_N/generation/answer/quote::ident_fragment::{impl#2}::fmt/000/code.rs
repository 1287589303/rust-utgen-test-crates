// Answer 0

#[derive(Debug)]
struct IdentFragment {
    value: String,
}

impl IdentFragment {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl std::fmt::Display for IdentFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let id = self.to_string();
        if let Some(id) = id.strip_prefix("r#") {
            fmt::Display::fmt(id, f)
        } else {
            fmt::Display::fmt(&id[..], f)
        }
    }
}

#[test]
fn test_fmt_with_r_prefix() {
    let ident = IdentFragment { value: String::from("r#my_identifier") };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", ident);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "my_identifier");
}

#[test]
fn test_fmt_without_r_prefix() {
    let ident = IdentFragment { value: String::from("my_identifier") };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", ident);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "my_identifier");
}

