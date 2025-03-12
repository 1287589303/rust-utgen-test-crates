// Answer 0

#[test]
fn test_nonzero_trailing_zeros_minimum() {
    let nonzero = NonZeroBitMaskWord::new_unchecked(1);
    let result = BitMask(0).nonzero_trailing_zeros(nonzero);
}

#[test]
fn test_nonzero_trailing_zeros_two() {
    let nonzero = NonZeroBitMaskWord::new_unchecked(2);
    let result = BitMask(0).nonzero_trailing_zeros(nonzero);
}

#[test]
fn test_nonzero_trailing_zeros_four() {
    let nonzero = NonZeroBitMaskWord::new_unchecked(4);
    let result = BitMask(0).nonzero_trailing_zeros(nonzero);
}

#[test]
fn test_nonzero_trailing_zeros_eight() {
    let nonzero = NonZeroBitMaskWord::new_unchecked(8);
    let result = BitMask(0).nonzero_trailing_zeros(nonzero);
}

#[test]
fn test_nonzero_trailing_zeros_max() {
    let nonzero = NonZeroBitMaskWord::new_unchecked(u64::MAX);
    let result = BitMask(0).nonzero_trailing_zeros(nonzero);
}

#[test]
#[should_panic]
fn test_nonzero_trailing_zeros_invalid() {
    let nonzero = NonZeroBitMaskWord::new_unchecked(0); 
    let result = BitMask(0).nonzero_trailing_zeros(nonzero);
}

