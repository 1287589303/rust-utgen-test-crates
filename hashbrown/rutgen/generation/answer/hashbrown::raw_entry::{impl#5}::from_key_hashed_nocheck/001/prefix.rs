// Answer 0

#[test]
fn test_from_key_hashed_nocheck_key_present() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = core::hash:: SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            core::hash::SipHasher::new()
        }
    }

    let map: HashMap<&str, u32, SimpleHasher> = [("a", 100), ("b", 200)].into();
    let key = "a";
    let hash = map.hasher().build_hasher().finish();
    let entry = map.raw_entry().from_key_hashed_nocheck(hash, &key);
}

#[test]
fn test_from_key_hashed_nocheck_key_absent() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = core::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            core::hash::SipHasher::new()
        }
    }

    let map: HashMap<&str, u32, SimpleHasher> = [("b", 200)].into();
    let key = "a";
    let hash = map.hasher().build_hasher().finish();
    let entry = map.raw_entry().from_key_hashed_nocheck(hash, &key);
}

#[test]
fn test_from_key_hashed_nocheck_hash_zero() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = core::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            core::hash::SipHasher::new()
        }
    }

    let map: HashMap<&str, u32, SimpleHasher> = [("b", 200)].into();
    let key = "a";
    let entry = map.raw_entry().from_key_hashed_nocheck(0, &key);
}

#[test]
fn test_from_key_hashed_nocheck_hash_max_u64() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = core::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            core::hash::SipHasher::new()
        }
    }

    let map: HashMap<&str, u32, SimpleHasher> = [("b", 200)].into();
    let key = "a";
    let entry = map.raw_entry().from_key_hashed_nocheck(u64::MAX, &key);
}

#[test]
fn test_from_key_hashed_nocheck_different_hasher() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct AnotherHasher;
    impl BuildHasher for AnotherHasher {
        type Hasher = core::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            core::hash::SipHasher::new()
        }
    }

    let map: HashMap<&str, u32, AnotherHasher> = [("x", 300), ("y", 400)].into();
    let key = "x";
    let hash = map.hasher().build_hasher().finish();
    let entry = map.raw_entry().from_key_hashed_nocheck(hash, &key);
}

