// Answer 0

#[test]
fn test_deref_mut_left() {
    struct L { value: i32 }
    impl DerefMut for L {
        fn deref_mut(&mut self) -> &mut Self { self }
    }
    
    struct R;
    impl DerefMut for R {
        fn deref_mut(&mut self) -> &mut Self { self }
    }

    let mut left_instance = L { value: 42 };
    let mut either_instance = Either::Left(left_instance);
    
    let result = either_instance.deref_mut();
    println!("{:?}", result.value);
}

#[test]
fn test_deref_mut_right() {
    struct L;
    impl DerefMut for L {
        fn deref_mut(&mut self) -> &mut Self { self }
    }
    
    struct R { value: String }
    impl DerefMut for R {
        fn deref_mut(&mut self) -> &mut Self { self }
    }

    let mut right_instance = R { value: String::from("test") };
    let mut either_instance = Either::Right(right_instance);
    
    let result = either_instance.deref_mut();
    println!("{:?}", result.value);
}

#[test]
fn test_deref_mut_empty_left() {
    struct L { value: Vec<i32> }
    impl DerefMut for L {
        fn deref_mut(&mut self) -> &mut Self { self }
    }
    
    struct R;
    impl DerefMut for R {
        fn deref_mut(&mut self) -> &mut Self { self }
    }

    let mut left_instance = L { value: Vec::new() };
    let mut either_instance = Either::Left(left_instance);
    
    let result = either_instance.deref_mut();
    println!("{:?}", result.value);
}

#[test]
fn test_deref_mut_empty_right() {
    struct L;
    impl DerefMut for L {
        fn deref_mut(&mut self) -> &mut Self { self }
    }
    
    struct R { value: Vec<i32> }
    impl DerefMut for R {
        fn deref_mut(&mut self) -> &mut Self { self }
    }

    let mut right_instance = R { value: Vec::new() };
    let mut either_instance = Either::Right(right_instance);
    
    let result = either_instance.deref_mut();
    println!("{:?}", result.value);
}

