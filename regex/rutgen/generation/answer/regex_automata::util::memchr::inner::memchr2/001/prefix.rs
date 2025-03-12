// Answer 0

#[test]
fn test_memchr2_with_empty_haystack() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_single_element_n1() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[1];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_single_element_n2() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[2];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_multi_element_haystack_n1_first() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[1, 3, 4];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_multi_element_haystack_n2_first() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[3, 2, 4];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_multi_element_haystack_n1_last() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[3, 4, 1];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_multi_element_haystack_n2_last() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[3, 4, 2];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_multi_element_haystack_both_present() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[3, 1, 4, 2];
    let _ = memchr2(n1, n2, haystack);
}

#[test]
fn test_memchr2_with_multi_element_haystack_both_present_reverse() {
    let n1 = 1;
    let n2 = 2;
    let haystack: &[u8] = &[2, 3, 4, 1];
    let _ = memchr2(n1, n2, haystack);
}

