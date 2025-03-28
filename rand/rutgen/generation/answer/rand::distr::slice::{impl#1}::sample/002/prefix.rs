// Answer 0

#[cfg(test)]
fn test_sample_out_of_bounds() {
    struct MockRng;

    impl crate::Rng for MockRng {
        fn gen_range(&mut self, low: usize, high: usize) -> usize {
            high  // Forces idx to be equal to slice.len()
        }
    }

    let slice = &[1, 2, 3];  // Non-empty slice of type T
    let num_choices = NonZeroUsize::new(1).unwrap();  // Valid num_choices
    let range = UniformUsize { low: 0, range: slice.len(), thresh: 0, mode64: false };  // Valid range setup

    let chooser = Choose { slice, range, num_choices };

    let mut rng = MockRng;

    chooser.sample(&mut rng);
}

#[cfg(test)]
fn test_sample_with_empty_slice() {
    struct MockRng;

    impl crate::Rng for MockRng {
        fn gen_range(&mut self, low: usize, high: usize) -> usize {
            high  // Forces idx to be equal to slice.len()
        }
    }

    let slice: &[i32] = &[];  // Empty slice
    let num_choices = NonZeroUsize::new(1).unwrap();  // Valid num_choices - to keep structure valid
    let range = UniformUsize { low: 0, range: 0, thresh: 0, mode64: false };  // Valid but edge case

    let chooser = Choose { slice, range, num_choices };

    let mut rng = MockRng;

    chooser.sample(&mut rng);
}

#[cfg(test)]
fn test_sample_single_element() {
    struct MockRng;

    impl crate::Rng for MockRng {
        fn gen_range(&mut self, low: usize, high: usize) -> usize {
            high  // Forces idx to be equal to slice.len()
        }
    }

    let slice = &[42];  // Single element slice
    let num_choices = NonZeroUsize::new(1).unwrap();  // Valid num_choices
    let range = UniformUsize { low: 0, range: slice.len(), thresh: 0, mode64: false };  // Valid range setup

    let chooser = Choose { slice, range, num_choices };

    let mut rng = MockRng;

    chooser.sample(&mut rng);
}

