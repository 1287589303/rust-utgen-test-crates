pub(crate) fn quit(byte: u8) -> StartError {
        StartError::Quit { byte }
    }