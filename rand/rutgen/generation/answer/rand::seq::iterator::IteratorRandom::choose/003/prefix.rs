// Answer 0

#[test]
fn test_choose_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl IteratorRandom for EmptyIterator {}

    let mut rng = rand::thread_rng();
    let empty_iter = EmptyIterator;
    let result = empty_iter.choose(&mut rng);
}

