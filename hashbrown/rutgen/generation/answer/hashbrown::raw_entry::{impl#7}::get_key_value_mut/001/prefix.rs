// Answer 0

#[test]
fn test_get_key_value_mut() {
    use std::rc::Rc;
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let key_one = Rc::new("a");
    let key_two = Rc::new("b");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    
    map.insert(key_one.clone(), 10);
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            assert!(Rc::strong_count(&inside_key) == 2); // Check the reference count
            *inside_value = 100;
        }
    }
}

#[test]
fn test_get_key_value_mut_with_different_key() {
    use std::rc::Rc;
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let key_one = Rc::new("a");
    let key_two = Rc::new("c");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    
    map.insert(key_one.clone(), 20);
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            *inside_key = key_two.clone();
            *inside_value = 200;
        }
    }
}

#[test]
fn test_get_key_value_mut_with_empty_table() {
    use std::rc::Rc;
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let key_one = Rc::new("d");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            *inside_value = 300; // This should panic as the key_one isn't in the map yet
        }
    }
}

