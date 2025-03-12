// Answer 0

#[test]
fn test_into_mut_with_occupied_entry() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);
    
    let value: &mut u32;
    match map.entry("poneyland") {
        Entry::Occupied(entry) => {
            value = entry.into_mut();
            *value += 10;
        },
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_into_mut_with_multiple_insertions() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);
    map.entry("unicornland").or_insert(20);
    
    let value: &mut u32;
    match map.entry("poneyland") {
        Entry::Occupied(entry) => {
            value = entry.into_mut();
            *value += 5;
        },
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
#[should_panic]
fn test_into_mut_with_non_existing_key() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let value: &mut u32;
    match map.entry("non_existing") {
        Entry::Occupied(_) => panic!(),
        Entry::Vacant(_) => {
            // Attempt to call into_mut on an entry that does not exist
            // This should not compile, demonstrating how occupation prevents this 
        },
    }
}

