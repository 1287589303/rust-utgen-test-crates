fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            TokenTreeIter::Compiler(tts) => tts.size_hint(),
            TokenTreeIter::Fallback(tts) => tts.size_hint(),
        }
    }