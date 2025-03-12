// Answer 0

#[test]
fn test_next_when_iter_has_elements_and_other_does_not() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    struct HashableElement(u32);

    impl PartialEq for HashableElement {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for HashableElement {}

    impl Hash for HashableElement {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let mut iter_elements = vec![HashableElement(1), HashableElement(2)].into_iter();
    let other_set: HashSet<HashableElement, DefaultHasher> = {
        let mut s = HashSet::new();
        s.insert(HashableElement(3));
        s.insert(HashableElement(4));
        s
    };

    let mut intersection = Intersection {
        iter: Iter { inner: iter_elements },
        other: &other_set,
    };

    let _result = intersection.next();
}

#[test]
fn test_next_when_iter_is_empty() {
    struct HashableElement(u32);

    impl PartialEq for HashableElement {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for HashableElement {}

    let iter_elements: Vec<HashableElement> = vec![];
    let other_set: HashSet<HashableElement, DefaultHasher> = {
        let mut s = HashSet::new();
        s.insert(HashableElement(1));
        s.insert(HashableElement(2));
        s
    };

    let mut intersection = Intersection {
        iter: Iter { inner: iter_elements.into_iter() },
        other: &other_set,
    };

    let _result = intersection.next();
}

#[test]
fn test_next_when_iter_has_elements_and_other_contains_them() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    struct HashableElement(u32);

    impl PartialEq for HashableElement {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for HashableElement {}

    impl Hash for HashableElement {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let mut iter_elements = vec![HashableElement(1), HashableElement(3)].into_iter();
    let other_set: HashSet<HashableElement, DefaultHasher> = {
        let mut s = HashSet::new();
        s.insert(HashableElement(1));
        s.insert(HashableElement(2));
        s
    };

    let mut intersection = Intersection {
        iter: Iter { inner: iter_elements },
        other: &other_set,
    };

    let _result = intersection.next();
}

