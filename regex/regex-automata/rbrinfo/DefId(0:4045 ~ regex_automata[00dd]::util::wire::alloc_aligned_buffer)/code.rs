pub(crate) fn alloc_aligned_buffer<T>(size: usize) -> (Vec<u8>, usize) {
    // NOTE: This is a kludge because there's no easy way to allocate a Vec<u8>
    // with an alignment guaranteed to be greater than 1. We could create a
    // Vec<u32>, but this cannot be safely transmuted to a Vec<u8> without
    // concern, since reallocing or dropping the Vec<u8> is UB (different
    // alignment than the initial allocation). We could define a wrapper type
    // to manage this for us, but it seems like more machinery than it's worth.
    let buf = vec![0; size];
    let align = core::mem::align_of::<T>();
    let address = buf.as_ptr().as_usize();
    if address % align == 0 {
        return (buf, 0);
    }
    // Let's try this again. We have to create a totally new alloc with
    // the maximum amount of bytes we might need. We can't just extend our
    // pre-existing 'buf' because that might create a new alloc with a
    // different alignment.
    let extra = align - 1;
    let mut buf = vec![0; size + extra];
    let address = buf.as_ptr().as_usize();
    // The code below handles the case where 'address' is aligned to T, so if
    // we got lucky and 'address' is now aligned to T (when it previously
    // wasn't), then we're done.
    if address % align == 0 {
        buf.truncate(size);
        return (buf, 0);
    }
    let padding = ((address & !(align - 1)).checked_add(align).unwrap())
        .checked_sub(address)
        .unwrap();
    assert!(padding <= 7, "padding of {} is bigger than 7", padding);
    assert!(
        padding <= extra,
        "padding of {} is bigger than extra {} bytes",
        padding,
        extra
    );
    buf.truncate(size + padding);
    assert_eq!(size + padding, buf.len());
    assert_eq!(
        0,
        buf[padding..].as_ptr().as_usize() % align,
        "expected end of initial padding to be aligned to {}",
        align,
    );
    (buf, padding)
}