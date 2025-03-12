pub(crate) fn padding_len(non_padding_len: usize) -> usize {
    (4 - (non_padding_len & 0b11)) & 0b11
}