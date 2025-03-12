// Answer 0

#[test]
fn test_create_captures_no_groups() {
    let regex = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(MockStrategy { group_info: GroupInfo::default() }),
            info: RegexInfo::default(),
        }),
        pool: CachePool::default(),
    };
    let captures = regex.create_captures();
}

#[test]
fn test_create_captures_one_group() {
    let group_info = GroupInfo(Arc::new(GroupInfoInner::new(1)));
    let regex = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(MockStrategy { group_info }),
            info: RegexInfo::default(),
        }),
        pool: CachePool::default(),
    };
    let captures = regex.create_captures();
}

#[test]
fn test_create_captures_multiple_named_groups() {
    let group_info = GroupInfo(Arc::new(GroupInfoInner::from_names(vec!["first".into(), "last".into()])));
    let regex = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(MockStrategy { group_info: group_info.clone() }),
            info: RegexInfo::default(),
        }),
        pool: CachePool::default(),
    };
    let captures = regex.create_captures();
}

#[test]
fn test_create_captures_shared_pool() {
    let shared_pool = CachePool::default();
    let group_info = GroupInfo(Arc::new(GroupInfoInner::new(2)));
    let regex1 = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(MockStrategy { group_info: group_info.clone() }),
            info: RegexInfo::default(),
        }),
        pool: shared_pool.clone(),
    };
    let regex2 = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(MockStrategy { group_info }),
            info: RegexInfo::default(),
        }),
        pool: shared_pool,
    };
    let captures1 = regex1.create_captures();
    let captures2 = regex2.create_captures();
}

#[derive(Clone)]
struct MockStrategy {
    group_info: GroupInfo,
}

impl Strategy for MockStrategy {
    fn group_info(&self) -> &GroupInfo {
        &self.group_info
    }
}

