// Answer 0

#[test]
fn test_and_modify_with_existing_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 41);

    let mut entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        elem: Bucket::new(&(map["poneyland"], 41)),
        table: &mut map.raw_table,
        hash_builder: &BuildHasherDefault::<DefaultHasher>::default(),
    });

    entry.and_modify(|_k, v| {
        *v += 1;
    });
}

#[test]
fn test_and_modify_with_another_existing_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 10);
    
    let mut entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        elem: Bucket::new(&(map["poneyland"], 10)),
        table: &mut map.raw_table,
        hash_builder: &BuildHasherDefault::<DefaultHasher>::default(),
    });

    entry.and_modify(|_k, v| {
        *v += 5;
    });
}

