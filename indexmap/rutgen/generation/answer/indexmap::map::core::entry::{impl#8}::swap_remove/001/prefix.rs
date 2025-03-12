// Answer 0

#[test]
fn swap_remove_multiple_entries() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    let indexed_entry = IndexedEntry::new(&mut map, 0);
    indexed_entry.swap_remove();
}

#[test]
fn swap_remove_last_entry() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    let indexed_entry = IndexedEntry::new(&mut map, 1); // last entry
    indexed_entry.swap_remove();
}

#[test]
fn swap_remove_single_entry() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(1, "only".to_string());
    let indexed_entry = IndexedEntry::new(&mut map, 0); // only entry
    indexed_entry.swap_remove();
} 

#[test]
fn swap_remove_edge_case() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    let indexed_entry = IndexedEntry::new(&mut map, 2); // last entry 
    indexed_entry.swap_remove();
}

