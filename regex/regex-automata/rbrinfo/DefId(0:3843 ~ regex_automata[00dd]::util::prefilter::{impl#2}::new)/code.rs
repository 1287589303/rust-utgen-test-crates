pub(crate) fn new<B: AsRef<[u8]>>(
        kind: MatchKind,
        needles: &[B],
    ) -> Option<Choice> {
        // An empty set means the regex matches nothing, so no sense in
        // building a prefilter.
        if needles.len() == 0 {
            debug!("prefilter building failed: found empty set of literals");
            return None;
        }
        // If the regex can match the empty string, then the prefilter
        // will by definition match at every position. This is obviously
        // completely ineffective.
        if needles.iter().any(|n| n.as_ref().is_empty()) {
            debug!("prefilter building failed: literals match empty string");
            return None;
        }
        // BREADCRUMBS: Perhaps the literal optimizer should special case
        // sequences of length two or three if the leading bytes of each are
        // "rare"? Or perhaps, if there are two or three total possible leading
        // bytes, regardless of the number of literals, and all are rare...
        // Then well, perhaps we should use memchr2 or memchr3 in those cases?
        if let Some(pre) = Memchr::new(kind, needles) {
            debug!("prefilter built: memchr");
            return Some(Choice::Memchr(pre));
        }
        if let Some(pre) = Memchr2::new(kind, needles) {
            debug!("prefilter built: memchr2");
            return Some(Choice::Memchr2(pre));
        }
        if let Some(pre) = Memchr3::new(kind, needles) {
            debug!("prefilter built: memchr3");
            return Some(Choice::Memchr3(pre));
        }
        if let Some(pre) = Memmem::new(kind, needles) {
            debug!("prefilter built: memmem");
            return Some(Choice::Memmem(pre));
        }
        if let Some(pre) = Teddy::new(kind, needles) {
            debug!("prefilter built: teddy");
            return Some(Choice::Teddy(pre));
        }
        if let Some(pre) = ByteSet::new(kind, needles) {
            debug!("prefilter built: byteset");
            return Some(Choice::ByteSet(pre));
        }
        if let Some(pre) = AhoCorasick::new(kind, needles) {
            debug!("prefilter built: aho-corasick");
            return Some(Choice::AhoCorasick(pre));
        }
        debug!("prefilter building failed: no strategy could be found");
        None
    }