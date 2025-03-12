pub fn encode_str(input: &str) -> Option<String> {
    if input.len() > u32::MAX as usize {
        return None;
    }
    let mut buf = String::with_capacity(input.len());
    encode_into::<_, _, ExternalCaller>(input.chars(), &mut buf)
        .ok()
        .map(|()| buf)
}