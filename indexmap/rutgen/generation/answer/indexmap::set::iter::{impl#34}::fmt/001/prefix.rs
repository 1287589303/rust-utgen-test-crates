// Answer 0

#[test]
fn test_intersection_debug_output_with_ints() {
    let hasher = std::collections::hash_map::RandomState::new();
    let mut set1 = super::IndexSet::<i32, _>::with_hasher(hasher.clone());
    set1.insert(1);
    set1.insert(2);
    let intersection = super::Intersection { iter: set1.iter(), other: &set1 };
    let _ = fmt::Debug::fmt(&intersection, &mut fmt::Formatter::new());
}

#[test]
fn test_intersection_debug_output_with_strings() {
    let hasher = std::collections::hash_map::RandomState::new();
    let mut set2 = super::IndexSet::<String, _>::with_hasher(hasher.clone());
    set2.insert("foo".to_string());
    set2.insert("bar".to_string());
    let intersection = super::Intersection { iter: set2.iter(), other: &set2 };
    let _ = fmt::Debug::fmt(&intersection, &mut fmt::Formatter::new());
}

#[test]
fn test_intersection_debug_output_with_empty_intersection() {
    let hasher = std::collections::hash_map::RandomState::new();
    let mut set3 = super::IndexSet::<i32, _>::with_hasher(hasher.clone());
    set3.insert(3);
    set3.insert(4);
    let intersection = super::Intersection { iter: set3.iter(), other: &set3 };
    let _ = fmt::Debug::fmt(&intersection, &mut fmt::Formatter::new());
}

#[test]
fn test_intersection_debug_output_with_hashable_struct() {
    #[derive(Debug, Eq, Hash)]
    struct MyStruct {
        id: i32,
    }
    
    let hasher = std::collections::hash_map::RandomState::new();
    let mut set4 = super::IndexSet::<MyStruct, _>::with_hasher(hasher.clone());
    set4.insert(MyStruct { id: 1 });
    let intersection = super::Intersection { iter: set4.iter(), other: &set4 };
    let _ = fmt::Debug::fmt(&intersection, &mut fmt::Formatter::new());
}

