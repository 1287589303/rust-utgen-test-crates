// Answer 0

#[test]
fn test_get_occupied_entry() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    let entry = set.entry("poneyland");
    let value = entry.get();
}

#[test]
fn test_get_vacant_entry() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    
    let entry = set.entry("horseland");
    let value = entry.get();
}

