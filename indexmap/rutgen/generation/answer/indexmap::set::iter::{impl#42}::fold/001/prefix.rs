// Answer 0

#[test]
fn test_fold_with_integer_accumulator() {
    let buckets = vec![Bucket::new(1), Bucket::new(2)];
    let iter = Iter { iter: buckets.as_slice().iter() };
    let other = IndexSet::new();
    let difference = Difference { iter, other: &other };
    let union = Union { iter: iter.chain(difference) };

    let init = 0;
    let result = union.fold(init, |acc, &item| acc + item);
}

#[test]
fn test_fold_with_string_accumulator() {
    let buckets = vec![Bucket::new("a"), Bucket::new("b")];
    let iter = Iter { iter: buckets.as_slice().iter() };
    let other = IndexSet::new();
    let difference = Difference { iter, other: &other };
    let union = Union { iter: iter.chain(difference) };

    let init = String::new();
    let result = union.fold(init, |mut acc, &item| {
        acc.push_str(item);
        acc
    });
}

#[test]
fn test_fold_with_custom_struct_accumulator() {
    #[derive(Default)]
    struct Accumulator {
        sum: i32,
        count: usize,
    }

    let buckets = vec![Bucket::new(1), Bucket::new(2)];
    let iter = Iter { iter: buckets.as_slice().iter() };
    let other = IndexSet::new();
    let difference = Difference { iter, other: &other };
    let union = Union { iter: iter.chain(difference) };

    let init = Accumulator::default();
    let result = union.fold(init, |mut acc, &item| {
        acc.sum += item;
        acc.count += 1;
        acc
    });
}

#[test]
fn test_fold_with_empty_union() {
    let buckets: Vec<Bucket<i32>> = Vec::new();
    let iter = Iter { iter: buckets.as_slice().iter() };
    let other = IndexSet::new();
    let difference = Difference { iter, other: &other };
    let union = Union { iter: iter.chain(difference) };

    let init = 10;
    let result = union.fold(init, |acc, &item| acc + item);
}

