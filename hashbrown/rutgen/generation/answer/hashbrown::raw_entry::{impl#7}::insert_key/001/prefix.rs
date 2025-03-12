// Answer 0

#[test]
fn test_insert_key_unoccupied_entry() {
    use hashbrown::hash_map::HashMap;
    use std::rc::Rc;

    let key = Rc::new("a");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&key) {
        RawEntryMut::Vacant(mut vacant) => {
            vacant.insert_key(key.clone());
        }
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_insert_key_occupied_entry() {
    use hashbrown::hash_map::HashMap;
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("a");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut occupied) => {
            let old_key = occupied.insert_key(key_two.clone());
            // The old key should still refer to the same memory.
            assert!(Rc::ptr_eq(&old_key, &key_one));
        }
    }
}

#[test]
fn test_insert_key_edge_case_empty_map() {
    use hashbrown::hash_map::HashMap;
    use std::rc::Rc;

    let key = Rc::new("test");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&key) {
        RawEntryMut::Vacant(mut vacant) => {
            vacant.insert_key(key.clone());
        }
        RawEntryMut::Occupied(_) => panic!(),
    }

    assert_eq!(map.get(&key), Some(&10));
}

#[test]
fn test_insert_key_edge_case_full_map() {
    use hashbrown::hash_map::HashMap;
    use std::rc::Rc;

    let key_one = Rc::new("full_key");
    let key_two = Rc::new("full_key");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Occupied(mut occupied) => {
            let old_key = occupied.insert_key(key_two.clone());
            assert!(Rc::ptr_eq(&old_key, &key_one));
        }
        RawEntryMut::Vacant(_) => panic!(),
    }
}

#[test]
fn test_insert_key_shared_ownership() {
    use hashbrown::hash_map::HashMap;
    use std::rc::Rc;

    let key_one = Rc::new("shared_a");
    let key_two = Rc::new("shared_a");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 20);
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Occupied(mut occupied) => {
            let old_key = occupied.insert_key(key_two.clone());
            assert!(Rc::ptr_eq(&old_key, &key_one));
        }
        RawEntryMut::Vacant(_) => panic!(),
    }
}

