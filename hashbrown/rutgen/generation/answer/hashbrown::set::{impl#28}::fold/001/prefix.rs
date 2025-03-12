// Answer 0

#[test]
fn test_fold_with_integers() {
    struct TestKeys<'a> {
        data: &'a [&'a str],
    }

    impl<'a> Keys<'a, &'a str, ()> {
        pub fn new(data: &'a [&'a str]) -> Self {
            Keys {
                inner: Iter { iter: TestKeys { data } },
            }
        }
    }

    let keys = TestKeys { data: &["key1", "key2", "key3"] };
    let keys_instance = Keys::new(keys.data);

    let result = keys_instance.fold(0, |acc, _key| acc + 1);
}

#[test]
fn test_fold_with_strings() {
    struct TestKeys<'a> {
        data: &'a [&'a str],
    }

    impl<'a> Keys<'a, &'a str, ()> {
        pub fn new(data: &'a [&'a str]) -> Self {
            Keys {
                inner: Iter { iter: TestKeys { data } },
            }
        }
    }

    let keys = TestKeys { data: &["key1", "key2"] };
    let keys_instance = Keys::new(keys.data);

    let result = keys_instance.fold(String::new(), |acc, key| acc + key);
}

#[test]
fn test_fold_with_structs() {
    struct TestKeys<'a> {
        data: &'a [&'a str],
    }

    #[derive(Default)]
    struct Accumulator {
        count: usize,
        keys: Vec<String>,
    }

    impl<'a> Keys<'a, &'a str, ()> {
        pub fn new(data: &'a [&'a str]) -> Self {
            Keys {
                inner: Iter { iter: TestKeys { data } },
            }
        }
    }

    let keys = TestKeys { data: &["key1", "key2"] };
    let keys_instance = Keys::new(keys.data);

    let result = keys_instance.fold(Accumulator::default(), |mut acc, key| {
        acc.count += 1;
        acc.keys.push(key.to_string());
        acc
    });
}

#[test]
fn test_fold_with_stateful_closure() {
    struct TestKeys<'a> {
        data: &'a [&'a str],
    }

    impl<'a> Keys<'a, &'a str, ()> {
        pub fn new(data: &'a [&'a str]) -> Self {
            Keys {
                inner: Iter { iter: TestKeys { data } },
            }
        }
    }

    let keys = TestKeys { data: &["key1", "key2", "key3"] };
    let keys_instance = Keys::new(keys.data);
    let mut counter = 0;

    let result = keys_instance.fold(0, |acc, _key| {
        counter += 1;
        acc + counter
    });
}

