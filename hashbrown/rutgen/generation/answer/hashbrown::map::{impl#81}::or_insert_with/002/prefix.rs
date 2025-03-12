// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_owned(), 3);

    let entry_ref = {
        // Simulating EntryRef::Occupied
        let key = "poneyland";
        let occupied_entry = map.entry_ref(key);
        occupied_entry
    };

    let _result: &mut u32 = entry_ref.or_insert_with(|| 10);
}

#[test]
fn test_or_insert_with_occupied_entry_modify() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("unicorn".to_owned(), 5);

    let entry_ref = {
        // Simulating EntryRef::Occupied
        let key = "unicorn";
        let occupied_entry = map.entry_ref(key);
        occupied_entry
    };

    let _result: &mut u32 = entry_ref.or_insert_with(|| 20);
}

#[test]
fn test_or_insert_with_occupied_entry_multiple_calls() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("dragon".to_owned(), 7);

    let entry_ref = {
        // Simulating EntryRef::Occupied
        let key = "dragon";
        let occupied_entry = map.entry_ref(key);
        occupied_entry
    };

    let _result: &mut u32 = entry_ref.or_insert_with(|| 13);
    let _result_again: &mut u32 = entry_ref.or_insert_with(|| 25);
}

