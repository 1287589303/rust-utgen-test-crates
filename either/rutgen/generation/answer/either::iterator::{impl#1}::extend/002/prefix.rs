// Answer 0

#[test]
fn test_extend_left_with_non_empty_iterable() {
    struct LeftCollection {
        items: Vec<i32>,
    }

    impl Extend<i32> for LeftCollection {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = i32>,
        {
            self.items.extend(iter);
        }
    }

    let mut left = Either::Left(LeftCollection { items: vec![1, 2, 3] });
    let iter = vec![4, 5, 6];
    left.extend(iter);
}

#[test]
fn test_extend_left_with_single_element_iterable() {
    struct LeftCollection {
        items: Vec<i32>,
    }

    impl Extend<i32> for LeftCollection {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = i32>,
        {
            self.items.extend(iter);
        }
    }

    let mut left = Either::Left(LeftCollection { items: vec![1] });
    let iter = vec![2];
    left.extend(iter);
}

#[test]
fn test_extend_left_with_large_iterable() {
    struct LeftCollection {
        items: Vec<i32>,
    }

    impl Extend<i32> for LeftCollection {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = i32>,
        {
            self.items.extend(iter);
        }
    }

    let mut left = Either::Left(LeftCollection { items: vec![10, 20, 30] });
    let iter = (1..=100).collect::<Vec<i32>>();
    left.extend(iter);
}

#[test]
fn test_extend_left_with_iterable_of_one_element() {
    struct LeftCollection {
        items: Vec<i32>,
    }

    impl Extend<i32> for LeftCollection {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = i32>,
        {
            self.items.extend(iter);
        }
    }

    let mut left = Either::Left(LeftCollection { items: vec![5] });
    let iter = vec![10, 15];
    left.extend(iter);
}

