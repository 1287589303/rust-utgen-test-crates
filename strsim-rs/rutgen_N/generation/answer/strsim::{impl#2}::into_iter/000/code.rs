// Answer 0

#[derive(Debug)]
struct MyStruct(String);

impl MyStruct {
    fn into_iter(self) -> std::str::Chars<'static> {
        self.0.chars()
    }
}

#[test]
fn test_into_iter() {
    let my_struct = MyStruct("hello".to_string());
    let iter = my_struct.into_iter();
    let result: Vec<char> = iter.collect();
    assert_eq!(result, vec!['h', 'e', 'l', 'l', 'o']);
}

#[test]
fn test_into_iter_empty_string() {
    let my_struct = MyStruct("".to_string());
    let iter = my_struct.into_iter();
    let result: Vec<char> = iter.collect();
    assert_eq!(result, Vec::<char>::new());
}

