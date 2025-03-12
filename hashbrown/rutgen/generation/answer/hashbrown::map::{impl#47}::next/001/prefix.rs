// Answer 0

#[test]
fn test_next_with_non_empty_iter() {
    struct TestKey(&'static str);
    struct TestValue(i32);

    impl Borrow<str> for TestKey {
        fn borrow(&self) -> &str {
            self.0
        }
    }

    impl Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestKey({})", self.0)
        }
    }

    impl Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestValue({})", self.0)
        }
    }

    let key_value_pair = (TestKey("key1"), TestValue(42));
    let mut iter = RawIter {
        iter: RawIterRange::new(&[key_value_pair], 1),
        items: 1,
    };

    let mut iter_wrapper = Iter {
        inner: iter,
        marker: PhantomData,
    };

    let result = iter_wrapper.next();
}

#[test]
fn test_next_with_multiple_elements() {
    struct TestKey(&'static str);
    struct TestValue(i32);

    impl Borrow<str> for TestKey {
        fn borrow(&self) -> &str {
            self.0
        }
    }

    impl Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestKey({})", self.0)
        }
    }

    impl Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestValue({})", self.0)
        }
    }

    let pairs = [
        (TestKey("key1"), TestValue(42)),
        (TestKey("key2"), TestValue(84)),
    ];
    let mut iter = RawIter {
        iter: RawIterRange::new(&pairs, pairs.len()),
        items: pairs.len(),
    };

    let mut iter_wrapper = Iter {
        inner: iter,
        marker: PhantomData,
    };

    let result = iter_wrapper.next();
    let result2 = iter_wrapper.next();
}

#[test]
fn test_next_with_different_types() {
    struct TestKey(u32);
    struct TestValue(bool);

    impl Borrow<u32> for TestKey {
        fn borrow(&self) -> &u32 {
            &self.0
        }
    }

    impl Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestKey({})", self.0)
        }
    }

    impl Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestValue({})", self.0)
        }
    }

    let key_value_pair = (TestKey(1), TestValue(true));
    let mut iter = RawIter {
        iter: RawIterRange::new(&[key_value_pair], 1),
        items: 1,
    };

    let mut iter_wrapper = Iter {
        inner: iter,
        marker: PhantomData,
    };

    let result = iter_wrapper.next();
}

