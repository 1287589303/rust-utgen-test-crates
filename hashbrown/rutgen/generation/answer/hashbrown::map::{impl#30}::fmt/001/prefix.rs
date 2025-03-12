// Answer 0

#[test]
fn test_fmt_empty_values() {
    struct Key;
    struct Value;

    let empty_values: Values<Key, Value> = Values {
        inner: Iter {
            inner: RawIter::new(),
            marker: PhantomData,
        }
    };

    let _ = empty_values.fmt(&mut fmt::Formatter::default());
}

#[test]
fn test_fmt_single_entry_values() {
    #[derive(Debug)]
    struct Key;
    #[derive(Debug)]
    struct Value;

    let single_entry_values: Values<Key, Value> = Values {
        inner: Iter {
            inner: RawIter::new_with_entries(vec![(Key, Value)]),
            marker: PhantomData,
        }
    };

    let _ = single_entry_values.fmt(&mut fmt::Formatter::default());
}

#[test]
fn test_fmt_multiple_entries_values() {
    #[derive(Debug)]
    struct Key;
    #[derive(Debug)]
    struct Value;

    let multiple_entries_values: Values<Key, Value> = Values {
        inner: Iter {
            inner: RawIter::new_with_entries(vec![(Key, Value), (Key, Value), (Key, Value)]),
            marker: PhantomData,
        }
    };

    let _ = multiple_entries_values.fmt(&mut fmt::Formatter::default());
}

