fn decode_without_base64<F, E>(
    encoded_body_plus_fragment: &str,
    mut write_bytes: F,
) -> Result<Option<FragmentIdentifier<'_>>, E>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    let bytes = encoded_body_plus_fragment.as_bytes();
    let mut slice_start = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        // We only need to look for 5 different "special" byte values.
        // For everything else we make slices as large as possible, borrowing the input,
        // in order to make fewer write_all() calls.
        if matches!(byte, b'%' | b'#' | b'\t' | b'\n' | b'\r') {
            // Write everything (if anything) "non-special" we’ve accumulated
            // before this special byte
            if i > slice_start {
                write_bytes(&bytes[slice_start..i])?;
                slice_start = i;
            }
            // Then deal with the special byte.
            match byte {
                b'%' => {
                    let l = bytes.get(i + 2).and_then(|&b| (b as char).to_digit(16));
                    let h = bytes.get(i + 1).and_then(|&b| (b as char).to_digit(16));
                    if let (Some(h), Some(l)) = (h, l) {
                        // '%' followed by two ASCII hex digits
                        let one_byte = h as u8 * 0x10 + l as u8;
                        write_bytes(&[one_byte])?;
                        slice_start = i + 3;
                    } else {
                        // Do nothing. Leave slice_start unchanged.
                        // The % sign will be part of the next slice.
                    }
                }

                b'#' => {
                    let fragment_start = i + 1;
                    let fragment = &encoded_body_plus_fragment[fragment_start..];
                    return Ok(Some(FragmentIdentifier(fragment)));
                }

                // Ignore over '\t' | '\n' | '\r'
                _ => slice_start = i + 1,
            }
        }
    }
    write_bytes(&bytes[slice_start..])?;
    Ok(None)
}