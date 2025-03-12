// Answer 0

#[test]
fn test_advance_mut_exact() {
    let mut buffer = Vec::with_capacity(10);
    unsafe { buffer.set_len(10); } 
    let remaining = buffer.capacity() - buffer.len(); // remaining is 10 now
    let cnt = remaining; // cnt is also 10, equal to remaining
    unsafe { buffer.advance_mut(cnt); } 
}

#[test]
fn test_advance_mut_one() {
    let mut buffer = Vec::with_capacity(1);
    unsafe { buffer.set_len(0); }
    let remaining = buffer.capacity() - buffer.len(); // remaining is 1
    let cnt = remaining; // cnt is also 1, equal to remaining
    unsafe { buffer.advance_mut(cnt); } 
} 

#[test]
fn test_advance_mut_large() {
    let mut buffer = Vec::with_capacity(20);
    unsafe { buffer.set_len(10); }
    let remaining = buffer.capacity() - buffer.len(); // remaining is 10
    let cnt = remaining; // cnt is also 10, equal to remaining
    unsafe { buffer.advance_mut(cnt); } 
}

