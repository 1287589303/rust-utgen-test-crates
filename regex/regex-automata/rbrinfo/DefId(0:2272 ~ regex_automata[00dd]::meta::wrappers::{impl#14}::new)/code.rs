pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid {
        ReverseHybrid(ReverseHybridEngine::new(info, nfarev))
    }