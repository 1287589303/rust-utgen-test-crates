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
#[derive(Default)]
pub(crate) struct Decoder {
    insertions: smallvec::SmallVec<[(usize, char); 59]>,
}
struct ExternalCaller;
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
impl Decoder {
    pub(crate) fn decode<'a, T: PunycodeCodeUnit + Copy, C: PunycodeCaller>(
        &'a mut self,
        input: &'a [T],
    ) -> Result<Decode<'a, T, C>, ()> {
        self.insertions.clear();
        let (base, input) = if let Some(position) = input
            .iter()
            .rposition(|c| c.is_delimiter())
        {
            (
                &input[..position],
                if position > 0 { &input[position + 1..] } else { input },
            )
        } else {
            (&input[..0], input)
        };
        if C::EXTERNAL_CALLER && !base.iter().all(|c| c.is_ascii()) {
            return Err(());
        }
        let base_len = base.len();
        let mut length = base_len as u32;
        let mut code_point = INITIAL_N;
        let mut bias = INITIAL_BIAS;
        let mut i = 0u32;
        let mut iter = input.iter();
        loop {
            let previous_i = i;
            let mut weight = 1;
            let mut k = BASE;
            let mut byte = match iter.next() {
                None => break,
                Some(byte) => byte,
            };
            loop {
                let digit = if let Some(digit) = byte.digit() {
                    digit
                } else {
                    return Err(());
                };
                let product = digit.checked_mul(weight).ok_or(())?;
                i = i.checked_add(product).ok_or(())?;
                let t = if k <= bias {
                    T_MIN
                } else if k >= bias + T_MAX {
                    T_MAX
                } else {
                    k - bias
                };
                if digit < t {
                    break;
                }
                weight = weight.checked_mul(BASE - t).ok_or(())?;
                k += BASE;
                byte = match iter.next() {
                    None => return Err(()),
                    Some(byte) => byte,
                };
            }
            bias = adapt(i - previous_i, length + 1, previous_i == 0);
            code_point = code_point.checked_add(i / (length + 1)).ok_or(())?;
            i %= length + 1;
            let c = match char::from_u32(code_point) {
                Some(c) => c,
                None => return Err(()),
            };
            for (idx, _) in &mut self.insertions {
                if *idx >= i as usize {
                    *idx += 1;
                }
            }
            self.insertions.push((i as usize, c));
            length += 1;
            i += 1;
        }
        self.insertions.sort_by_key(|(i, _)| *i);
        Ok(Decode {
            base: base.iter(),
            insertions: &self.insertions,
            inserted: 0,
            position: 0,
            len: base_len + self.insertions.len(),
            phantom: PhantomData::<C>,
        })
    }
}
pub fn decode(input: &str) -> Option<Vec<char>> {
    Some(
        Decoder::default().decode::<u8, ExternalCaller>(input.as_bytes()).ok()?.collect(),
    )
}
