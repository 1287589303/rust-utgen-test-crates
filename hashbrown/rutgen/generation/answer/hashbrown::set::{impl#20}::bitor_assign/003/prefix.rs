// Answer 0

#[test]
fn test_bitor_assign_empty_rhs() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    let b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };

    a |= &b;
}

#[test]
fn test_bitor_assign_non_empty_rhs() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    let b: HashSet<i32> = {
        let mut temp = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
        temp.insert(1);
        temp.insert(2);
        temp.insert(3);
        temp
    };

    a |= &b;
}

#[test]
fn test_bitor_assign_with_existing_nonintersecting_elements() {
    let mut a: HashSet<i32> = {
        let mut temp = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
        temp.insert(4);
        temp.insert(5);
        temp
    };
    let b: HashSet<i32> = {
        let mut temp = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
        temp.insert(1);
        temp.insert(2);
        temp.insert(3);
        temp
    };

    a |= &b;
}

#[test]
fn test_bitor_assign_rhs_with_duplicates() {
    let mut a: HashSet<i32> = {
        let mut temp = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
        temp.insert(1);
        temp.insert(3);
        temp
    };
    let b: HashSet<i32> = {
        let mut temp = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
        temp.insert(2);
        temp.insert(2); // Duplicate
        temp.insert(3); // Existing in a
        temp
    };

    a |= &b;
}

