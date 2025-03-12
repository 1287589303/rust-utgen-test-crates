pub(crate) fn u32s_to_state_ids(slice: &[u32]) -> &[StateID] {
    // SAFETY: This is safe because StateID is defined to have the same memory
    // representation as a u32 (it is repr(transparent)). While not every u32
    // is a "valid" StateID, callers are not permitted to rely on the validity
    // of StateIDs for memory safety. It can only lead to logical errors. (This
    // is why StateID::new_unchecked is safe.)
    unsafe {
        core::slice::from_raw_parts(
            slice.as_ptr().cast::<StateID>(),
            slice.len(),
        )
    }
}