pub fn decode_to_string(input: &str) -> Option<String> {
    Some(
        Decoder::default()
            .decode::<u8, ExternalCaller>(input.as_bytes())
            .ok()?
            .collect(),
    )
}