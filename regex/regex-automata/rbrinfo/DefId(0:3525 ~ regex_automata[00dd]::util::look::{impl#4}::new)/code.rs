pub fn new() -> LookMatcher {
        LookMatcher { lineterm: DebugByte(b'\n') }
    }