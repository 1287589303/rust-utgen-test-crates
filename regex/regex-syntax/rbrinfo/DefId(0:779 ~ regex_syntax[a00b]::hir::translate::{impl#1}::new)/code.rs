pub fn new() -> TranslatorBuilder {
        TranslatorBuilder {
            utf8: true,
            line_terminator: b'\n',
            flags: Flags::default(),
        }
    }