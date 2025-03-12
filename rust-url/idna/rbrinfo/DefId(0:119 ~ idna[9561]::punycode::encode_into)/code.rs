pub(crate) fn encode_into<I, W, C>(input: I, output: &mut W) -> Result<(), PunycodeEncodeError>
where
    I: Iterator<Item = char> + Clone,
    W: Write + ?Sized,
    C: PunycodeCaller,
{
    // Handle "basic" (ASCII) code points. They are encoded as-is.
    let (mut input_length, mut basic_length) = (0u32, 0);
    for c in input.clone() {
        input_length = input_length
            .checked_add(1)
            .ok_or(PunycodeEncodeError::Overflow)?;
        if c.is_ascii() {
            output.write_char(c)?;
            basic_length += 1;
        }
    }

    if !C::EXTERNAL_CALLER {
        // We should never get an overflow here with the internal caller being
        // length-limited, but let's check anyway once here trusting the math
        // from  RFC 3492 section 6.4 and then omit the overflow checks in the
        // loop below.
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
        // All code points < code_point have been handled already.
        // Find the next larger one.
        let min_code_point = input
            .clone()
            .map(|c| c as u32)
            .filter(|&c| c >= code_point)
            .min()
            .unwrap();
        // Increase delta to advance the decoder’s <code_point,i> state to <min_code_point,0>
        if C::EXTERNAL_CALLER {
            let product = (min_code_point - code_point)
                .checked_mul(processed + 1)
                .ok_or(PunycodeEncodeError::Overflow)?;
            delta = delta
                .checked_add(product)
                .ok_or(PunycodeEncodeError::Overflow)?;
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
                // Represent delta as a generalized variable-length integer:
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