// Answer 0

#[test]
fn test_either_with_left() {
    struct TestContext {
        values: Vec<i32>,
    }

    let mut context = TestContext { values: Vec::new() };
    let left_value = Either::Left(5);
    
    left_value.either_with(&mut context,
        |ctx, integer| ctx.values.push(integer),
        |_, _| panic!("Should not call the right handler"),
    );

    // The context should contain 5
    let expected = vec![5];
    assert_eq!(context.values, expected);
}

#[test]
fn test_either_with_right() {
    struct TestContext {
        values: Vec<i32>,
    }

    let mut context = TestContext { values: Vec::new() };
    let right_value = Either::Right(2.9);
    
    right_value.either_with(&mut context,
        |_, _| panic!("Should not call the left handler"),
        |ctx, real| ctx.values.push(f64::round(real) as i32),
    );

    // The context should contain 3
    let expected = vec![3];
    assert_eq!(context.values, expected);
}

