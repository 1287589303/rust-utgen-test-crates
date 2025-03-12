// Answer 0

#[test]
fn test_from_array_with_distinct_integers() {
    let set: hashbrown::HashSet<_> = [1, 2, 3, 4].into();
}

#[test]
fn test_from_array_with_repeated_integers() {
    let set: hashbrown::HashSet<_> = [1, 1, 2, 3].into();
}

#[test]
fn test_from_array_with_empty() {
    let set: hashbrown::HashSet<i32> = [].into();
}

#[test]
fn test_from_array_with_distinct_strings() {
    let set: hashbrown::HashSet<&str> = ["hello", "world", "rust"].into();
}

#[test]
fn test_from_array_with_repeated_strings() {
    let set: hashbrown::HashSet<&str> = ["hello", "hello", "rust"].into();
}

#[test]
fn test_from_array_with_custom_structs() {
    #[derive(Eq, Hash)]
    struct CustomStruct {
        id: i32,
    }
    
    let set: hashbrown::HashSet<_> = [
        CustomStruct { id: 1 },
        CustomStruct { id: 2 },
    ].into();
}

#[test]
fn test_from_array_with_repeated_custom_structs() {
    #[derive(Eq, Hash)]
    struct CustomStruct {
        id: i32,
    }
    
    let set: hashbrown::HashSet<_> = [
        CustomStruct { id: 1 },
        CustomStruct { id: 1 },
        CustomStruct { id: 2 },
    ].into();
}

