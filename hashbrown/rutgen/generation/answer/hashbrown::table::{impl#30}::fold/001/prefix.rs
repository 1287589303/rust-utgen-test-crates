// Answer 0

#[test]
fn test_fold_with_integer_init() {
    struct TestType(i32);

    let iterator = IterHash { 
        inner: RawIterHash { inner: RawIterHashInner::new(), _marker: PhantomData },
        marker: PhantomData 
    };
    
    let init_value = 0;
    
    let result = iterator.fold(init_value, |acc, item| acc + item.0);
}

#[test]
fn test_fold_with_string_init() {
    struct TestType(String);

    let iterator = IterHash { 
        inner: RawIterHash { inner: RawIterHashInner::new(), _marker: PhantomData },
        marker: PhantomData 
    };
    
    let init_value = String::from("Start - ");
    
    let result = iterator.fold(init_value, |acc, item| format!("{}{}", acc, item.0));
}

#[test]
fn test_fold_with_empty_iterator() {
    struct TestType(i32);

    let iterator = IterHash { 
        inner: RawIterHash { inner: RawIterHashInner::new_empty(), _marker: PhantomData },
        marker: PhantomData 
    };
    
    let init_value = 5;
    
    let result = iterator.fold(init_value, |acc, _item| acc + 1);
}

#[test]
fn test_fold_with_struct_init() {
    struct TestType(i32);
    struct Accumulator {
        sum: i32,
        count: i32,
    }

    let iterator = IterHash { 
        inner: RawIterHash { inner: RawIterHashInner::new(), _marker: PhantomData },
        marker: PhantomData 
    };
    
    let init_value = Accumulator { sum: 0, count: 0 };
    
    let result = iterator.fold(init_value, |mut acc, item| {
        acc.sum += item.0;
        acc.count += 1;
        acc
    });
}

#[test]
fn test_fold_with_boundary_value() {
    struct TestType(i32);

    let iterator = IterHash { 
        inner: RawIterHash { inner: RawIterHashInner::new_with_boundary(), _marker: PhantomData },
        marker: PhantomData 
    };
    
    let init_value = 1;
    
    let result = iterator.fold(init_value, |acc, item| acc * item.0);
}

