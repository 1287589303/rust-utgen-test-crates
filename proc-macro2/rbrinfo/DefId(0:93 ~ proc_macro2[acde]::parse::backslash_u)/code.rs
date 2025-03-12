fn backslash_u<I>(chars: &mut I) -> Result<char, Reject>
where
    I: Iterator<Item = (usize, char)>,
{
    next_ch!(chars @ '{');
    let mut value = 0;
    let mut len = 0;
    for (_, ch) in chars {
        let digit = match ch {
            '0'..='9' => ch as u8 - b'0',
            'a'..='f' => 10 + ch as u8 - b'a',
            'A'..='F' => 10 + ch as u8 - b'A',
            '_' if len > 0 => continue,
            '}' if len > 0 => return char::from_u32(value).ok_or(Reject),
            _ => break,
        };
        if len == 6 {
            break;
        }
        value *= 0x10;
        value += u32::from(digit);
        len += 1;
    }
    Err(Reject)
}