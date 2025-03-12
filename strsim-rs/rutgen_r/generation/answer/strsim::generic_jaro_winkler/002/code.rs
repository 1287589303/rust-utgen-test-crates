// Answer 0

#[test]
fn test_generic_jaro_winkler_sim_below_threshold() {
    struct IterA<'a> {
        data: &'a [char],
    }

    struct IterB<'a> {
        data: &'a [char],
    }

    impl<'a> IntoIterator for &'a IterA<'a> {
        type Item = char;
        type IntoIter = std::slice::Iter<'a, char>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    impl<'a> IntoIterator for &'a IterB<'a> {
        type Item = char;
        type IntoIter = std::slice::Iter<'a, char>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    let a = IterA { data: &['a', 'b', 'c', 'd'] };
    let b = IterB { data: &['e', 'f', 'g', 'h'] };
    
    let result = generic_jaro_winkler(&a, &b);
    assert_eq!(result, 0.7);
}

