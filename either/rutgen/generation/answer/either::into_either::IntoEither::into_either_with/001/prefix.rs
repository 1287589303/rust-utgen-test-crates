// Answer 0

#[test]
fn test_into_either_with_u8_even() {
    fn is_even(x: &u8) -> bool {
        x % 2 == 0
    }
    
    let x: u8 = 0;
    x.into_either_with(is_even);
    
    let y: u8 = 1;
    y.into_either_with(is_even);
}

#[test]
fn test_into_either_with_u8_odd() {
    fn is_even(x: &u8) -> bool {
        x % 2 == 0
    }

    let x: u8 = 255;
    x.into_either_with(is_even);

    let y: u8 = 254;
    y.into_either_with(is_even);
}

#[test]
fn test_into_either_with_string_true() {
    fn is_length_even(s: &String) -> bool {
        s.len() % 2 == 0
    }
    
    let s1 = String::from("test");
    s1.into_either_with(is_length_even);

    let s2 = String::from("hello");
    s2.into_either_with(is_length_even);
}

#[test]
fn test_into_either_with_string_false() {
    fn is_length_even(s: &String) -> bool {
        s.len() % 2 == 0
    }
    
    let s1 = String::from("");
    s1.into_either_with(is_length_even);

    let s2 = String::from("odd");
    s2.into_either_with(is_length_even);
}

#[test]
fn test_into_either_with_custom_struct() {
    #[derive(Serialize, Deserialize)]
    struct MyStruct {
        value: u8,
    }

    fn is_value_greater_than_100(s: &MyStruct) -> bool {
        s.value > 100
    }

    let x = MyStruct { value: 50 };
    x.into_either_with(is_value_greater_than_100);

    let y = MyStruct { value: 150 };
    y.into_either_with(is_value_greater_than_100);
}

