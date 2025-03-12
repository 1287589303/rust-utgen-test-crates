// Answer 0

#[test]
fn test_remove_occupied_entry_string() {
    let mut set: hashbrown::HashSet<String> = hashbrown::HashSet::new();
    set.insert("test".to_string());
    if let hashbrown::hash_set::Entry::Occupied(o) = set.entry("test") {
        let value = o.remove();
        drop(value);
    }
}

#[test]
fn test_remove_occupied_entry_integer() {
    let mut set: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set.insert(42);
    if let hashbrown::hash_set::Entry::Occupied(o) = set.entry(&42) {
        let value = o.remove();
        drop(value);
    }
}

#[test]
fn test_remove_occupied_entry_f32() {
    let mut set: hashbrown::HashSet<f32> = hashbrown::HashSet::new();
    set.insert(3.14);
    if let hashbrown::hash_set::Entry::Occupied(o) = set.entry(&3.14) {
        let value = o.remove();
        drop(value);
    }
}

#[test]
fn test_remove_occupied_entry_tuple() {
    let mut set: hashbrown::HashSet<(i32, &str)> = hashbrown::HashSet::new();
    set.insert((1, "one"));
    if let hashbrown::hash_set::Entry::Occupied(o) = set.entry(&(1, "one")) {
        let value = o.remove();
        drop(value);
    }
}

