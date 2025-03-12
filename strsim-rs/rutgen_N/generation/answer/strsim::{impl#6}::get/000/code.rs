// Answer 0

#[derive(Debug)]
struct CustomStruct {
    extended_ascii: Vec<u8>,
    map: std::collections::HashMap<u32, u32>,
}

impl CustomStruct {
    fn new() -> Self {
        CustomStruct {
            extended_ascii: vec![0; 256],
            map: std::collections::HashMap::new(),
        }
    }

    fn get(&self, key: char) -> u32 {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            self.extended_ascii[usize::from(val_u8)] as u32
        } else {
            *self.map.get(&value).unwrap_or(&0)
        }
    }
}

#[test]
fn test_get_with_ascii_character() {
    let mut custom_struct = CustomStruct::new();
    custom_struct.extended_ascii[65] = 1; // 'A'
    
    assert_eq!(custom_struct.get('A'), 1);
    assert_eq!(custom_struct.get('B'), 0); // Uninitialized
}

#[test]
fn test_get_with_non_ascii_character() {
    let mut custom_struct = CustomStruct::new();
    custom_struct.map.insert(256, 2); // Non-ASCII character
    
    assert_eq!(custom_struct.get('Ā'), 2);
    assert_eq!(custom_struct.get('Ē'), 0); // Uninitialized
}

#[test]
fn test_get_with_boundary_character() {
    let custom_struct = CustomStruct::new();
    
    assert_eq!(custom_struct.get('\u{FF}'), 0); // Upper boundary of ASCII
}

#[test]
#[should_panic(expected = "we check the bounds above")]
fn test_get_with_invalid_conversion() {
    let custom_struct = CustomStruct::new();
    let _ = custom_struct.get('\u{100}'); // Should panic due to out of bounds
}

