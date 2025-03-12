// Answer 0

#[test]
fn test_into_key_value_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }
    *inside_key = key_two.clone();
    *inside_value = 100;
}

#[test]
fn test_into_key_value_changes_map() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("b");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 20);

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }
    *inside_key = key_two.clone();
    *inside_value = 200;
}

#[test]
fn test_into_key_value_with_shared_references() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("c");
    let key_two = Rc::new("c");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 30);

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }
    *inside_key = key_two.clone();
    *inside_value = 300;
}

