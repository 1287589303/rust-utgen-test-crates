// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::cmp::min;

    fn flat_index(i: usize, j: usize, width: usize) -> usize {
        i * width + j
    }

    #[test]
    fn test_generic_damerau_levenshtein_a_empty() {
        let a: &[i32] = &[];
        let b: &[i32] = &[1, 2, 3];
        assert_eq!(3, generic_damerau_levenshtein(a, b));
    }

    #[test]
    fn test_generic_damerau_levenshtein_b_empty() {
        let a: &[i32] = &[1, 2, 3];
        let b: &[i32] = &[];
        assert_eq!(3, generic_damerau_levenshtein(a, b));
    }

    #[test]
    fn test_generic_damerau_levenshtein_both_empty() {
        let a: &[i32] = &[];
        let b: &[i32] = &[];
        assert_eq!(0, generic_damerau_levenshtein(a, b));
    }

    #[test]
    fn test_generic_damerau_levenshtein_single_insert() {
        let a: &[i32] = &[1];
        let b: &[i32] = &[1, 2];
        assert_eq!(1, generic_damerau_levenshtein(a, b));
    }

    #[test]
    fn test_generic_damerau_levenshtein_single_delete() {
        let a: &[i32] = &[1, 2];
        let b: &[i32] = &[1];
        assert_eq!(1, generic_damerau_levenshtein(a, b));
    }

    #[test]
    fn test_generic_damerau_levenshtein_single_substitution() {
        let a: &[i32] = &[1];
        let b: &[i32] = &[2];
        assert_eq!(1, generic_damerau_levenshtein(a, b));
    }

    #[test]
    fn test_generic_damerau_levenshtein_transposition() {
        let a: &[i32] = &[1, 2];
        let b: &[i32] = &[2, 1];
        assert_eq!(1, generic_damerau_levenshtein(a, b));
    }

    #[test]
    fn test_generic_damerau_levenshtein_multiple_operations() {
        let a: &[i32] = &[1, 3, 2];
        let b: &[i32] = &[2, 1];
        assert_eq!(3, generic_damerau_levenshtein(a, b));
    }
}

