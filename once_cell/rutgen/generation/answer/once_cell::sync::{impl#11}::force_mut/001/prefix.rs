// Answer 0

#[test]
fn test_force_mut_with_initialization() {
    struct Test;
    
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| {
            let mut value = 42;
            &mut value
        })),
    };
    
    let _result = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_different_function() {
    struct Test;
    
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| {
            let mut value = 100;
            &mut value
        })),
    };
    
    let _result = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_zero_value() {
    struct Test;
    
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| {
            let mut value = 0;
            &mut value
        })),
    };
    
    let _result = Lazy::force_mut(&mut lazy);
}

#[test]
#[should_panic]
fn test_force_mut_panics_when_poisoned() {
    struct Test;

    let mut lazy = Lazy {
        cell: OnceCell::with_value(5),
        init: Cell::new(None),
    };

    let _result = Lazy::force_mut(&mut lazy);
}

