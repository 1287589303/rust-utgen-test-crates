// Answer 0

#[test]
fn test_index_mut_valid_range() {
    let mut data: &mut [u8] = &mut [1, 2, 3, 4, 5];
    let index = Span { start: 1, end: 4 };
    let result = data.index_mut(index);
}

#[test]
fn test_index_mut_full_range() {
    let mut data: &mut [u8] = &mut [10, 20, 30, 40, 50];
    let index = Span { start: 0, end: 5 };
    let result = data.index_mut(index);
}

#[test]
fn test_index_mut_single_element() {
    let mut data: &mut [u8] = &mut [100, 200, 300];
    let index = Span { start: 2, end: 3 };
    let result = data.index_mut(index);
}

#[test]
fn test_index_mut_boundary_cases() {
    let mut data: &mut [u8] = &mut [5, 10, 15];
    let index = Span { start: 0, end: 1 };
    let result = data.index_mut(index);
    
    let index_full = Span { start: 0, end: 3 };
    let result_full = data.index_mut(index_full);
}

