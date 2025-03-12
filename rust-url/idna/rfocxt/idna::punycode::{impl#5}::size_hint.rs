use alloc::{string::String, vec::Vec};
use core::char;
use core::fmt::Write;
use core::marker::PhantomData;
const BASE: u32 = 36;
const T_MIN: u32 = 1;
const T_MAX: u32 = 26;
const SKEW: u32 = 38;
const DAMP: u32 = 700;
const INITIAL_BIAS: u32 = 72;
const INITIAL_N: u32 = 0x80;
pub(crate) trait PunycodeCodeUnit {
    fn is_delimiter(&self) -> bool;
    fn is_ascii(&self) -> bool;
    fn digit(&self) -> Option<u32>;
    fn char(&self) -> char;
    fn char_ascii_lower_case(&self) -> char;
}
pub(crate) trait PunycodeCaller {
    const EXTERNAL_CALLER: bool;
}
pub(crate) struct Decode<'a, T, C>
where
    T: PunycodeCodeUnit + Copy,
    C: PunycodeCaller,
{
    base: core::slice::Iter<'a, T>,
    pub(crate) insertions: &'a [(usize, char)],
    inserted: usize,
    position: usize,
    len: usize,
    phantom: PhantomData<C>,
}
impl<T: PunycodeCodeUnit + Copy, C: PunycodeCaller> Iterator for Decode<'_, T, C> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.insertions.get(self.inserted) {
                Some((pos, c)) if *pos == self.position => {
                    self.inserted += 1;
                    self.position += 1;
                    return Some(*c);
                }
                _ => {}
            }
            if let Some(c) = self.base.next() {
                self.position += 1;
                return Some(
                    if C::EXTERNAL_CALLER { c.char() } else { c.char_ascii_lower_case() },
                );
            } else if self.inserted >= self.insertions.len() {
                return None;
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len - self.position;
        (len, Some(len))
    }
}
