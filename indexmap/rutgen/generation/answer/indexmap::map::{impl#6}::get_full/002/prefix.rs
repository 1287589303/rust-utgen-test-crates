// Answer 0

#[test]
fn test_get_full_existing_key() {
    let mut map = IndexMap::<i32, &str, RandomState>::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let key = &2;
    let result = map.get_full(key);
}

#[test]
fn test_get_full_first_entry() {
    let mut map = IndexMap::<i32, &str, RandomState>::new();
    map.insert(0, "zero");
    map.insert(1, "one");

    let key = &0;
    let result = map.get_full(key);
}

#[test]
fn test_get_full_last_entry() {
    let mut map = IndexMap::<i32, &str, RandomState>::new();
    map.insert(10, "ten");
    map.insert(20, "twenty");

    let key = &20;
    let result = map.get_full(key);
}

#[test]
fn test_get_full_middle_entry() {
    let mut map = IndexMap::<i32, &str, RandomState>::new();
    map.insert(5, "five");
    map.insert(15, "fifteen");
    map.insert(25, "twenty-five");

    let key = &15;
    let result = map.get_full(key);
}

