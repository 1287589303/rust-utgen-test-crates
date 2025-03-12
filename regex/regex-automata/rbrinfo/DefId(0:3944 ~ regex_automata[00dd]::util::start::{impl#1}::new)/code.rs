pub(crate) fn new(lookm: &LookMatcher) -> StartByteMap {
        let mut map = [Start::NonWordByte; 256];
        map[usize::from(b'\n')] = Start::LineLF;
        map[usize::from(b'\r')] = Start::LineCR;
        map[usize::from(b'_')] = Start::WordByte;

        let mut byte = b'0';
        while byte <= b'9' {
            map[usize::from(byte)] = Start::WordByte;
            byte += 1;
        }
        byte = b'A';
        while byte <= b'Z' {
            map[usize::from(byte)] = Start::WordByte;
            byte += 1;
        }
        byte = b'a';
        while byte <= b'z' {
            map[usize::from(byte)] = Start::WordByte;
            byte += 1;
        }

        let lineterm = lookm.get_line_terminator();
        // If our line terminator is normal, then it is already handled by
        // the LineLF and LineCR configurations. But if it's weird, then we
        // overwrite whatever was there before for that terminator with a
        // special configuration. The trick here is that if the terminator
        // is, say, a word byte like `a`, then callers seeing this start
        // configuration need to account for that and build their DFA state as
        // if it *also* came from a word byte.
        if lineterm != b'\r' && lineterm != b'\n' {
            map[usize::from(lineterm)] = Start::CustomLineTerminator;
        }
        StartByteMap { map }
    }