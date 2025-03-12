// Answer 0

#[test]
fn test_next_valid_case() {
    struct AccelArray {
        data: [AccelTy; 2],
    }

    impl AsRef<[AccelTy]> for AccelArray {
        fn as_ref(&self) -> &[AccelTy] {
            &self.data
        }
    }
    
    let accels = Accels { accels: AccelArray { data: [1, 2] } };
    let mut iter = IterAccels { accels: &accels, i: 0 };
    let result = iter.next();
}

#[test]
fn test_next_second_element() {
    struct AccelArray {
        data: [AccelTy; 3],
    }

    impl AsRef<[AccelTy]> for AccelArray {
        fn as_ref(&self) -> &[AccelTy] {
            &self.data
        }
    }
    
    let accels = Accels { accels: AccelArray { data: [10, 20, 30] } };
    let mut iter = IterAccels { accels: &accels, i: 1 };
    let result = iter.next();
}

#[test]
fn test_next_boundary_case() {
    struct AccelArray {
        data: [AccelTy; 1],
    }

    impl AsRef<[AccelTy]> for AccelArray {
        fn as_ref(&self) -> &[AccelTy] {
            &self.data
        }
    }
    
    let accels = Accels { accels: AccelArray { data: [100] } };
    let mut iter = IterAccels { accels: &accels, i: 0 };
    let result = iter.next();
}

