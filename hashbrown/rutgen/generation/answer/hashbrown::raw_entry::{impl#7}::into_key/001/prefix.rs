// Answer 0

#[test]
fn test_into_key_with_valid_rc_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_two.clone();
}

#[test]
fn test_into_key_with_replacement() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("b");
    let key_two = Rc::new("c");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 20);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_two.clone();
}

#[test]
fn test_into_key_with_same_reference_count() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("d");
    let key_two = Rc::new("d");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 30);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_two.clone();
}

#[test]
fn test_into_key_with_different_values() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("e");
    let key_two = Rc::new("f");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 40);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_two.clone();
}

