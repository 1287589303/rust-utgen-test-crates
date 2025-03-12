pub(crate) fn u32s_to_pattern_ids(slice: &[u32]) -> &[PatternID] {
    // SAFETY: This is safe because PatternID is defined to have the same
    // memory representation as a u32 (it is repr(transparent)). While not
    // every u32 is a "valid" PatternID, callers are not permitted to rely
    // on the validity of PatternIDs for memory safety. It can only lead to
    // logical errors. (This is why PatternID::new_unchecked is safe.)
    unsafe {
        core::slice::from_raw_parts(
            slice.as_ptr().cast::<PatternID>(),
            slice.len(),
        )
    }
}