fn child(&self) -> &'a Hir {
        match *self {
            Frame::Repetition(rep) => &rep.sub,
            Frame::Capture(capture) => &capture.sub,
            Frame::Concat { head, .. } => head,
            Frame::Alternation { head, .. } => head,
        }
    }