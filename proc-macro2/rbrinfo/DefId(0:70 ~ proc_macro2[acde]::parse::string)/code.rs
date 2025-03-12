fn string(input: Cursor) -> Result<Cursor, Reject> {
    if let Ok(input) = input.parse("\"") {
        cooked_string(input)
    } else if let Ok(input) = input.parse("r") {
        raw_string(input)
    } else {
        Err(Reject)
    }
}