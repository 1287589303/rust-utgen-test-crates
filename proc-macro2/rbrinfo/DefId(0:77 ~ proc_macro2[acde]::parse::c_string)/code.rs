fn c_string(input: Cursor) -> Result<Cursor, Reject> {
    if let Ok(input) = input.parse("c\"") {
        cooked_c_string(input)
    } else if let Ok(input) = input.parse("cr") {
        raw_c_string(input)
    } else {
        Err(Reject)
    }
}