use crate::RngCore;
pub(crate) trait Observable: Copy {
    type Bytes: Sized + AsRef<[u8]>;
    fn to_le_bytes(self) -> Self::Bytes;
}
pub(crate) fn fill_via_chunks<T: Observable>(
    src: &[T],
    dest: &mut [u8],
) -> (usize, usize) {
    let size = core::mem::size_of::<T>();
    let mut dest = dest.chunks_exact_mut(size);
    let mut src = src.iter();
    let zipped = dest.by_ref().zip(src.by_ref());
    let num_chunks = zipped.len();
    zipped.for_each(|(dest, src)| dest.copy_from_slice(src.to_le_bytes().as_ref()));
    let byte_len = num_chunks * size;
    if let Some(src) = src.next() {
        let dest = dest.into_remainder();
        let n = dest.len();
        if n > 0 {
            dest.copy_from_slice(&src.to_le_bytes().as_ref()[..n]);
            return (num_chunks + 1, byte_len + n);
        }
    }
    (num_chunks, byte_len)
}
