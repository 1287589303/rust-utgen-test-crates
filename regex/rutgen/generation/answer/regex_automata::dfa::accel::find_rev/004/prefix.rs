// Answer 0

#[test]
fn test_find_rev_two_needles_valid_haystack() {
    let needles: [u8; 2] = [10, 20];
    let haystack: &[u8] = &[5, 10, 15, 20, 25, 30];
    let at: usize = 6; // valid index
    
    find_rev(&needles, haystack, at);
}

#[test]
#[should_panic(expected = "invalid needles length: 2")]
fn test_find_rev_two_needles_invalid_length() {
    let needles: [u8; 2] = [10, 20];
    let haystack: &[u8] = &[5, 10, 15, 20, 25, 30];
    let at: usize = 1; // valid index in the lower part of the haystack
    
    let invalid_needles: &[u8] = &[]; // should panic due to empty needles
    find_rev(invalid_needles, haystack, at);
} 

#[test]
fn test_find_rev_two_needles_last_occurrence() {
    let needles: [u8; 2] = [10, 20];
    let haystack: &[u8] = &[5, 10, 15, 20, 10, 30];
    let at: usize = 6; // valid index
    
    find_rev(&needles, haystack, at);
}

#[test]
fn test_find_rev_two_needles_boundaries() {
    let needles: [u8; 2] = [0, 255];
    let haystack: &[u8] = &[0, 1, 2, 3, 255];
    let at: usize = 5; // valid index
    
    find_rev(&needles, haystack, at);
}

