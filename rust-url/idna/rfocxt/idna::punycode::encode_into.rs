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
pub(crate) enum PunycodeEncodeError {
    Overflow,
    Sink,
}
pub(crate) fn encode_into<I, W, C>(
    input: I,
    output: &mut W,
) -> Result<(), PunycodeEncodeError>
where
    I: Iterator<Item = char> + Clone,
    W: Write + ?Sized,
    C: PunycodeCaller,
{
    let (mut input_length, mut basic_length) = (0u32, 0);
    for c in input.clone() {
        input_length = input_length.checked_add(1).ok_or(PunycodeEncodeError::Overflow)?;
        if c.is_ascii() {
            output.write_char(c)?;
            basic_length += 1;
        }
    }
    if !C::EXTERNAL_CALLER {
        let len_plus_one = input_length
            .checked_add(1)
            .ok_or(PunycodeEncodeError::Overflow)?;
        len_plus_one
            .checked_mul(u32::from(char::MAX) - INITIAL_N)
            .ok_or(PunycodeEncodeError::Overflow)?;
    }
    if basic_length > 0 {
        output.write_char('-')?;
    }
    let mut code_point = INITIAL_N;
    let mut delta = 0u32;
    let mut bias = INITIAL_BIAS;
    let mut processed = basic_length;
    while processed < input_length {
        let min_code_point = input
            .clone()
            .map(|c| c as u32)
            .filter(|&c| c >= code_point)
            .min()
            .unwrap();
        if C::EXTERNAL_CALLER {
            let product = (min_code_point - code_point)
                .checked_mul(processed + 1)
                .ok_or(PunycodeEncodeError::Overflow)?;
            delta = delta.checked_add(product).ok_or(PunycodeEncodeError::Overflow)?;
        } else {
            delta += (min_code_point - code_point) * (processed + 1);
        }
        code_point = min_code_point;
        for c in input.clone() {
            let c = c as u32;
            if c < code_point {
                if C::EXTERNAL_CALLER {
                    delta = delta.checked_add(1).ok_or(PunycodeEncodeError::Overflow)?;
                } else {
                    delta += 1;
                }
            }
            if c == code_point {
                let mut q = delta;
                let mut k = BASE;
                loop {
                    let t = if k <= bias {
                        T_MIN
                    } else if k >= bias + T_MAX {
                        T_MAX
                    } else {
                        k - bias
                    };
                    if q < t {
                        break;
                    }
                    let value = t + ((q - t) % (BASE - t));
                    output.write_char(value_to_digit(value))?;
                    q = (q - t) / (BASE - t);
                    k += BASE;
                }
                output.write_char(value_to_digit(q))?;
                bias = adapt(delta, processed + 1, processed == basic_length);
                delta = 0;
                processed += 1;
            }
        }
        delta += 1;
        code_point += 1;
    }
    Ok(())
}
#[inline]
fn adapt(mut delta: u32, num_points: u32, first_time: bool) -> u32 {
    delta /= if first_time { DAMP } else { 2 };
    delta += delta / num_points;
    let mut k = 0;
    while delta > ((BASE - T_MIN) * T_MAX) / 2 {
        delta /= BASE - T_MIN;
        k += BASE;
    }
    k + (((BASE - T_MIN + 1) * delta) / (delta + SKEW))
}
#[inline]
fn value_to_digit(value: u32) -> char {
    match value {
        0..=25 => (value as u8 + b'a') as char,
        26..=35 => (value as u8 - 26 + b'0') as char,
        _ => panic!(),
    }
}
