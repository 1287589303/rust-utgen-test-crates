// Answer 0

#[test]
fn test_move_index_boundary_low() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(0, 0), (1, 1), (2, 2)] }
        }
        
        fn borrow_mut(&mut self) -> RefMut<usize, usize> {
            RefMut::new(&mut self.entries, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    let entry = IndexedEntry::new(&mut map, 0); // current index 0
    entry.move_index(1); // move to index 1
}

#[test]
fn test_move_index_boundary_high() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(0, 0), (1, 1), (2, 2)] }
        }
        
        fn borrow_mut(&mut self) -> RefMut<usize, usize> {
            RefMut::new(&mut self.entries, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    let entry = IndexedEntry::new(&mut map, 1); // current index 1
    entry.move_index(2); // move to index 2
}

#[test]
fn test_move_index_to_first() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(0, 0), (1, 1), (2, 2)] }
        }
        
        fn borrow_mut(&mut self) -> RefMut<usize, usize> {
            RefMut::new(&mut self.entries, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    let entry = IndexedEntry::new(&mut map, 1); // current index 1
    entry.move_index(0); // move to index 0
}

#[test]
fn test_move_index_to_adjacent_lower() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(0, 0), (1, 1), (2, 2)] }
        }
        
        fn borrow_mut(&mut self) -> RefMut<usize, usize> {
            RefMut::new(&mut self.entries, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    let entry = IndexedEntry::new(&mut map, 2); // current index 2
    entry.move_index(1); // move to index 1
}

#[test]
fn test_move_index_to_adjacent_higher() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(0, 0), (1, 1), (2, 2)] }
        }
        
        fn borrow_mut(&mut self) -> RefMut<usize, usize> {
            RefMut::new(&mut self.entries, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    let entry = IndexedEntry::new(&mut map, 0); // current index 0
    entry.move_index(1); // move to index 1
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_too_high() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(0, 0), (1, 1), (2, 2)] }
        }
        
        fn borrow_mut(&mut self) -> RefMut<usize, usize> {
            RefMut::new(&mut self.entries, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    let entry = IndexedEntry::new(&mut map, 1); // current index 1
    entry.move_index(3); // out of bounds
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_too_low() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(0, 0), (1, 1), (2, 2)] }
        }
        
        fn borrow_mut(&mut self) -> RefMut<usize, usize> {
            RefMut::new(&mut self.entries, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    let entry = IndexedEntry::new(&mut map, 0); // current index 0
    entry.move_index(usize::MAX); // out of bounds
}

