// Answer 0

#[test]
fn test_rfold_with_empty_right_iterator() {
    struct SimpleAcc {
        value: i32,
    }

    let right_iter = std::iter::empty::<i32>();
    let either = Either::Right(right_iter);
    let init = SimpleAcc { value: 0 };
    let result = either.rfold(init, |acc, item| {
        SimpleAcc { value: acc.value + item }
    });
}

#[test]
fn test_rfold_with_non_empty_right_iterator() {
    struct SimpleAcc {
        value: i32,
    }

    let right_iter = vec![1, 2, 3].into_iter().rev();
    let either = Either::Right(right_iter);
    let init = SimpleAcc { value: 0 };
    let result = either.rfold(init, |acc, item| {
        SimpleAcc { value: acc.value + item }
    });
}

#[test]
fn test_rfold_with_complex_accumulation() {
    #[derive(Debug)]
    struct ComplexAcc {
        total: i32,
        count: usize,
    }

    let right_iter = vec![10, 20, 30].into_iter().rev();
    let either = Either::Right(right_iter);
    let init = ComplexAcc { total: 0, count: 0 };
    let result = either.rfold(init, |mut acc, item| {
        acc.total += item;
        acc.count += 1;
        acc
    });
}

#[test]
fn test_rfold_with_identity_function() {
    let right_iter = vec![5, 10, 15].into_iter().rev();
    let either = Either::Right(right_iter);
    let init = 100;
    let result = either.rfold(init, |acc, item| acc);
}

#[test]
fn test_rfold_with_edge_case_init() {
    let right_iter = std::iter::once(42);
    let either = Either::Right(right_iter);
    let init = 0;
    let result = either.rfold(init, |acc, item| acc + item);
}

