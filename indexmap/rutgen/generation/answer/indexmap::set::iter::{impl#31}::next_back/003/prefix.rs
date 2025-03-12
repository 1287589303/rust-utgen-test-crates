// Answer 0

#[test]
fn test_next_back_returns_none_when_contains_is_false() {
    struct Bucket<T> {
        value: T,
    }
    
    struct IndexSet<T, S> {
        elements: Vec<T>,
        _marker: std::marker::PhantomData<S>,
    }

    impl<T: Eq + Hash, S: BuildHasher> IndexSet<T, S> {
        fn contains(&self, item: &T) -> bool {
            self.elements.contains(item)
        }
    }

    struct Iter<'a, T> {
        items: &'a [Bucket<T>],
        current: usize,
    }

    impl<'a, T> Iter<'a, T> {
        fn new(items: &'a [Bucket<T>]) -> Self {
            Iter { items, current: items.len() }
        }

        fn next_back(&mut self) -> Option<&'a T> {
            if self.current == 0 {
                None
            } else {
                self.current -= 1;
                Some(&self.items[self.current].value)
            }
        }
    }

    struct Intersection<'a, T, S> {
        iter: Iter<'a, T>,
        other: &'a IndexSet<T, S>,
    }

    let buckets = [
        Bucket { value: 1 },
        Bucket { value: 2 },
        Bucket { value: 3 },
    ];
    let index_set = IndexSet { elements: vec![4, 5], _marker: std::marker::PhantomData };

    let mut intersection = Intersection {
        iter: Iter::new(&buckets),
        other: &index_set,
    };

    let _item1 = intersection.iter.next_back(); // Should return Some(3)
    let _item2 = intersection.iter.next_back(); // Should return Some(2)
    let _item3 = intersection.iter.next_back(); // Should return Some(1)
    
    let result = intersection.iter.next_back(); // Should return None
}

