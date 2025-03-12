type Chunk = u32;
use core::{mem, ops};
const ASCII_RANGE_LEN: usize = 0x80;
const BITS_PER_CHUNK: usize = 8 * mem::size_of::<Chunk>();
pub const CONTROLS: &AsciiSet = &AsciiSet {
    mask: [!0_u32, 0, 0, 1 << (0x7F_u32 % 32)],
};
pub const NON_ALPHANUMERIC: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b'-')
    .add(b'.')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'_')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}')
    .add(b'~');
#[derive(Debug, PartialEq, Eq)]
pub struct AsciiSet {
    mask: [Chunk; ASCII_RANGE_LEN / BITS_PER_CHUNK],
}
impl AsciiSet {
    pub const EMPTY: AsciiSet = AsciiSet {
        mask: [0; ASCII_RANGE_LEN / BITS_PER_CHUNK],
    };
    pub(crate) const fn contains(&self, byte: u8) -> bool {
        let chunk = self.mask[byte as usize / BITS_PER_CHUNK];
        let mask = 1 << (byte as usize % BITS_PER_CHUNK);
        (chunk & mask) != 0
    }
    pub(crate) fn should_percent_encode(&self, byte: u8) -> bool {
        !byte.is_ascii() || self.contains(byte)
    }
    pub const fn add(&self, byte: u8) -> Self {
        let mut mask = self.mask;
        mask[byte as usize / BITS_PER_CHUNK] |= 1 << (byte as usize % BITS_PER_CHUNK);
        AsciiSet { mask }
    }
    pub const fn remove(&self, byte: u8) -> Self {
        let mut mask = self.mask;
        mask[byte as usize / BITS_PER_CHUNK] &= !(1 << (byte as usize % BITS_PER_CHUNK));
        AsciiSet { mask }
    }
    pub const fn union(&self, other: Self) -> Self {
        let mask = [
            self.mask[0] | other.mask[0],
            self.mask[1] | other.mask[1],
            self.mask[2] | other.mask[2],
            self.mask[3] | other.mask[3],
        ];
        AsciiSet { mask }
    }
    pub const fn complement(&self) -> Self {
        let mask = [!self.mask[0], !self.mask[1], !self.mask[2], !self.mask[3]];
        AsciiSet { mask }
    }
}
