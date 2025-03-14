// Answer 0

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    fn new(value: &str) -> Result<Self, String> {
        if value.is_empty() {
            Err("Value cannot be empty".to_string())
        } else {
            Ok(MyStruct)
        }
    }
}

trait MyTrait {
    type Error;

    fn try_from(value: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl MyTrait for MyStruct {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[test]
fn test_try_from_non_empty_string() {
    let result = MyStruct::try_from("valid input");
    assert!(result.is_ok());
}

#[test]
fn test_try_from_empty_string() {
    let result = MyStruct::try_from("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Value cannot be empty");
}

