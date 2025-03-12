fn parse_ipv6addr(input: &str) -> ParseResult<Ipv6Addr> {
    let input = input.as_bytes();
    let len = input.len();
    let mut is_ip_v4 = false;
    let mut pieces = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut piece_pointer = 0;
    let mut compress_pointer = None;
    let mut i = 0;

    if len < 2 {
        return Err(ParseError::InvalidIpv6Address);
    }

    if input[0] == b':' {
        if input[1] != b':' {
            return Err(ParseError::InvalidIpv6Address);
        }
        i = 2;
        piece_pointer = 1;
        compress_pointer = Some(1);
    }

    while i < len {
        if piece_pointer == 8 {
            return Err(ParseError::InvalidIpv6Address);
        }
        if input[i] == b':' {
            if compress_pointer.is_some() {
                return Err(ParseError::InvalidIpv6Address);
            }
            i += 1;
            piece_pointer += 1;
            compress_pointer = Some(piece_pointer);
            continue;
        }
        let start = i;
        let end = cmp::min(len, start + 4);
        let mut value = 0u16;
        while i < end {
            match (input[i] as char).to_digit(16) {
                Some(digit) => {
                    value = value * 0x10 + digit as u16;
                    i += 1;
                }
                None => break,
            }
        }
        if i < len {
            match input[i] {
                b'.' => {
                    if i == start {
                        return Err(ParseError::InvalidIpv6Address);
                    }
                    i = start;
                    if piece_pointer > 6 {
                        return Err(ParseError::InvalidIpv6Address);
                    }
                    is_ip_v4 = true;
                }
                b':' => {
                    i += 1;
                    if i == len {
                        return Err(ParseError::InvalidIpv6Address);
                    }
                }
                _ => return Err(ParseError::InvalidIpv6Address),
            }
        }
        if is_ip_v4 {
            break;
        }
        pieces[piece_pointer] = value;
        piece_pointer += 1;
    }

    if is_ip_v4 {
        if piece_pointer > 6 {
            return Err(ParseError::InvalidIpv6Address);
        }
        let mut numbers_seen = 0;
        while i < len {
            if numbers_seen > 0 {
                if numbers_seen < 4 && (i < len && input[i] == b'.') {
                    i += 1
                } else {
                    return Err(ParseError::InvalidIpv6Address);
                }
            }

            let mut ipv4_piece = None;
            while i < len {
                let digit = match input[i] {
                    c @ b'0'..=b'9' => c - b'0',
                    _ => break,
                };
                match ipv4_piece {
                    None => ipv4_piece = Some(digit as u16),
                    Some(0) => return Err(ParseError::InvalidIpv6Address), // No leading zero
                    Some(ref mut v) => {
                        *v = *v * 10 + digit as u16;
                        if *v > 255 {
                            return Err(ParseError::InvalidIpv6Address);
                        }
                    }
                }
                i += 1;
            }

            pieces[piece_pointer] = if let Some(v) = ipv4_piece {
                pieces[piece_pointer] * 0x100 + v
            } else {
                return Err(ParseError::InvalidIpv6Address);
            };
            numbers_seen += 1;

            if numbers_seen == 2 || numbers_seen == 4 {
                piece_pointer += 1;
            }
        }

        if numbers_seen != 4 {
            return Err(ParseError::InvalidIpv6Address);
        }
    }

    if i < len {
        return Err(ParseError::InvalidIpv6Address);
    }

    match compress_pointer {
        Some(compress_pointer) => {
            let mut swaps = piece_pointer - compress_pointer;
            piece_pointer = 7;
            while swaps > 0 {
                pieces.swap(piece_pointer, compress_pointer + swaps - 1);
                swaps -= 1;
                piece_pointer -= 1;
            }
        }
        _ => {
            if piece_pointer != 8 {
                return Err(ParseError::InvalidIpv6Address);
            }
        }
    }
    Ok(Ipv6Addr::new(
        pieces[0], pieces[1], pieces[2], pieces[3], pieces[4], pieces[5], pieces[6], pieces[7],
    ))
}