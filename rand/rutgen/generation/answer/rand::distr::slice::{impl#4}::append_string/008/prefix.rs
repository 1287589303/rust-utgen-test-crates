// Answer 0

#[test]
fn test_append_string_with_full_slice_length() {
    struct TestRng;

    impl crate::Rng for TestRng {
        // Implement necessary methods for Rng trait here
    }

    let chars: Vec<char> = ('a'..'z').collect();
    let slice: &[char] = &chars;
    let num_choices = NonZeroUsize::new(5).unwrap();
    let range = UniformUsize { low: 0, range: 1, thresh: 0 };
    let chooser = Choose { slice, range, num_choices };
    
    let mut rng = TestRng;
    let mut string = String::new();
    let len = 100; // Satisfies len >= 100
    
    chooser.append_string(&mut rng, &mut string, len);
}

#[test]
fn test_append_string_with_exact_char_limit() {
    struct TestRng;

    impl crate::Rng for TestRng {
        // Implement necessary methods for Rng trait here
    }

    let chars: Vec<char> = ('a'..'z').collect();
    let slice: &[char] = &chars;
    let num_choices = NonZeroUsize::new(5).unwrap();
    let range = UniformUsize { low: 0, range: 1, thresh: 0 };
    let chooser = Choose { slice, range, num_choices };

    let mut rng = TestRng;
    let mut string = String::new();
    let len = 200; // Length is on the boundary
    
    chooser.append_string(&mut rng, &mut string, len);
}

