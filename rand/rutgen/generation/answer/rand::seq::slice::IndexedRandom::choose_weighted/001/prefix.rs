// Answer 0

#[test]
fn test_choose_weighted_empty_slice() {
    struct EmptySlice;
    
    impl Index<usize> for EmptySlice {
        type Output = ();
        
        fn index(&self, _: usize) -> &Self::Output {
            panic!("Index out of bounds");
        }
    }
    
    impl IndexedRandom for EmptySlice {
        fn len(&self) -> usize { 0 }
        
        #[inline]
        fn is_empty(&self) -> bool { true }
    }
    
    let mut rng = rand::rng();
    let empty_slice = EmptySlice;
    let result: Result<_, WeightError> = empty_slice.choose_weighted(&mut rng, |&_: &()| 1);
}

#[test]
fn test_choose_weighted_invalid_weights() {
    struct WeightedSlice {
        elements: Vec<char>,
        weights: Vec<i32>,
    }

    impl Index<usize> for WeightedSlice {
        type Output = char;
        
        fn index(&self, index: usize) -> &Self::Output {
            &self.elements[index]
        }
    }
    
    impl IndexedRandom for WeightedSlice {
        fn len(&self) -> usize { self.elements.len() }
        
        #[inline]
        fn is_empty(&self) -> bool { self.elements.is_empty() }
    }
    
    let elements = vec!['a', 'b', 'c', 'd'];
    let weights = vec![1, -1, 1, 0]; // Invalid negative weight
    let weighted_slice = WeightedSlice {
        elements,
        weights,
    };
    
    let mut rng = rand::rng();
    let result: Result<_, WeightError> = weighted_slice.choose_weighted(&mut rng, |&elem| {
        match elem {
            'a' => weights[0],
            'b' => weights[1],
            'c' => weights[2],
            'd' => weights[3],
            _ => 0,
        }
    });
}

#[test]
fn test_choose_weighted_overflow() {
    struct OverflowSlice {
        elements: Vec<char>,
    }

    impl Index<usize> for OverflowSlice {
        type Output = char;
        
        fn index(&self, index: usize) -> &Self::Output {
            &self.elements[index]
        }
    }

    impl IndexedRandom for OverflowSlice {
        fn len(&self) -> usize { self.elements.len() }
        
        #[inline]
        fn is_empty(&self) -> bool { self.elements.is_empty() }
    }

    let elements = vec!['a', 'b', 'c'];
    let weights = vec![i32::MAX, i32::MAX, i32::MAX]; // Weights that will exceed sum limits
    let overflow_slice = OverflowSlice {
        elements,
    };
    
    let mut rng = rand::rng();
    let result: Result<_, WeightError> = overflow_slice.choose_weighted(&mut rng, |&elem| {
        match elem {
            'a' => weights[0],
            'b' => weights[1],
            'c' => weights[2],
            _ => 0,
        }
    });
}

#[test]
fn test_choose_weighted_insufficient_non_zero_weights() {
    struct NonZeroSlice {
        elements: Vec<char>,
    }

    impl Index<usize> for NonZeroSlice {
        type Output = char;
        
        fn index(&self, index: usize) -> &Self::Output {
            &self.elements[index]
        }
    }

    impl IndexedRandom for NonZeroSlice {
        fn len(&self) -> usize { self.elements.len() }
        
        #[inline]
        fn is_empty(&self) -> bool { self.elements.is_empty() }
    }

    let elements = vec!['a', 'b', 'c'];
    let weights = vec![0, 0, 0]; // All weights zero
    let non_zero_slice = NonZeroSlice {
        elements,
    };
    
    let mut rng = rand::rng();
    let result: Result<_, WeightError> = non_zero_slice.choose_weighted(&mut rng, |&elem| {
        match elem {
            'a' => weights[0],
            'b' => weights[1],
            'c' => weights[2],
            _ => 0,
        }
    });
}

