fn backslash_x_nonzero<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, char)>,
{
    let first = next_ch!(chars @ '0'..='9' | 'a'..='f' | 'A'..='F');
    let second = next_ch!(chars @ '0'..='9' | 'a'..='f' | 'A'..='F');
    if first == '0' && second == '0' {
        Err(Reject)
    } else {
        Ok(())
    }
}