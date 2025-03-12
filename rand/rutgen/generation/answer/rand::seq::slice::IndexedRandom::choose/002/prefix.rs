// Answer 0

#[test]
fn choose_non_empty_array() {
    struct TestArray(&'static [i32]);
    
    impl Index<usize> for TestArray {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl IndexedRandom for TestArray {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    
    let choices = TestArray(&[1, 2, 3, 4, 5]);
    let mut rng = rand::rng();
    let _ = choices.choose(&mut rng);
}

#[test]
fn choose_non_empty_vector() {
    struct TestVector(Vec<i32>);
    
    impl Index<usize> for TestVector {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl IndexedRandom for TestVector {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    
    let choices = TestVector(vec![10, 20, 30, 40, 50]);
    let mut rng = rand::rng();
    let _ = choices.choose(&mut rng);
}

#[test]
fn choose_non_empty_single_element() {
    struct SingleElement(&'static [i32]);
    
    impl Index<usize> for SingleElement {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl IndexedRandom for SingleElement {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    
    let choices = SingleElement(&[99]);
    let mut rng = rand::rng();
    let _ = choices.choose(&mut rng);
}

