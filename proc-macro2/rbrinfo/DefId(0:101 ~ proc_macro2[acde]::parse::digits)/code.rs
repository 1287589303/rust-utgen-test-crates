fn digits(mut input: Cursor) -> Result<Cursor, Reject> {
    let base = if input.starts_with("0x") {
        input = input.advance(2);
        16
    } else if input.starts_with("0o") {
        input = input.advance(2);
        8
    } else if input.starts_with("0b") {
        input = input.advance(2);
        2
    } else {
        10
    };

    let mut len = 0;
    let mut empty = true;
    for b in input.bytes() {
        match b {
            b'0'..=b'9' => {
                let digit = (b - b'0') as u64;
                if digit >= base {
                    return Err(Reject);
                }
            }
            b'a'..=b'f' => {
                let digit = 10 + (b - b'a') as u64;
                if digit >= base {
                    break;
                }
            }
            b'A'..=b'F' => {
                let digit = 10 + (b - b'A') as u64;
                if digit >= base {
                    break;
                }
            }
            b'_' => {
                if empty && base == 10 {
                    return Err(Reject);
                }
                len += 1;
                continue;
            }
            _ => break,
        }
        len += 1;
        empty = false;
    }
    if empty {
        Err(Reject)
    } else {
        Ok(input.advance(len))
    }
}