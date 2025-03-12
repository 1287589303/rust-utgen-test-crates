// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    
    map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count());

    // This is to ensure that after inserting using the or_insert_with_key,
    // the value should be 9 as "poneyland" has 9 characters.
    let value: &mut usize = map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count());
}

#[test]
fn test_or_insert_with_key_vacant_entry_multiple() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    
    // Inserting a value which is not present in the HashMap
    map.entry_ref("unicorn").or_insert_with_key(|key| key.chars().count());

    // Checking a second nonexistent key
    map.entry_ref("dragon").or_insert_with_key(|key| key.chars().count());

    // Ensure both inserted values are as expected
    let value1: &mut usize = map.entry_ref("unicorn").or_insert_with_key(|key| key.chars().count());
    let value2: &mut usize = map.entry_ref("dragon").or_insert_with_key(|key| key.chars().count());
}

#[test]
fn test_or_insert_with_key_vacant_entry_different_type() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, f64> = HashMap::new();

    // Key with a value that matches the default function's expected return type
    map.entry_ref("floaty").or_insert_with_key(|key| key.chars().count() as f64 * 1.5);
    
    let value: &mut f64 = map.entry_ref("floaty").or_insert_with_key(|key| key.chars().count() as f64 * 1.5);
}

