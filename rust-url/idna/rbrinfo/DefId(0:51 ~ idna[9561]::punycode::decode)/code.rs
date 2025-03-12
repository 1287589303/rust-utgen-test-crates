pub fn decode(input: &str) -> Option<Vec<char>> {
    Some(
        Decoder::default()
            .decode::<u8, ExternalCaller>(input.as_bytes())
            .ok()?
            .collect(),
    )
}