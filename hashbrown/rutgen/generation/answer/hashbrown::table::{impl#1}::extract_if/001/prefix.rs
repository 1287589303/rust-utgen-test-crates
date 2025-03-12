// Answer 0

#[test]
fn test_extract_if_empty() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let extractor = table.extract_if(|_v| false);
}

#[test]
fn test_extract_if_with_no_matches() {
    let mut table = HashTable::new_in(Global);
    table.insert_unique(1, 10, |v| v.to_owned());
    table.insert_unique(2, 20, |v| v.to_owned());
    let extractor = table.extract_if(|_v| false);
}

#[test]
fn test_extract_if_with_all_matches() {
    let mut table = HashTable::new_in(Global);
    for x in 0..8 {
        table.insert_unique(x, x, |v| v.to_owned());
    }
    let extractor = table.extract_if(|v| *v % 2 == 0);
}

#[test]
fn test_extract_if_with_some_matches() {
    let mut table = HashTable::new_in(Global);
    for x in 0..8 {
        table.insert_unique(x, x, |v| v.to_owned());
    }
    let extractor = table.extract_if(|v| *v % 2 == 0);
} 

#[test]
fn test_extract_if_single_element() {
    let mut table = HashTable::new_in(Global);
    table.insert_unique(1, 5, |v| v.to_owned());
    let extractor = table.extract_if(|v| *v > 2);
}

#[test]
fn test_extract_if_two_elements_one_match() {
    let mut table = HashTable::new_in(Global);
    table.insert_unique(1, 3, |v| v.to_owned());
    table.insert_unique(2, 6, |v| v.to_owned());
    let extractor = table.extract_if(|v| *v > 5);
}

